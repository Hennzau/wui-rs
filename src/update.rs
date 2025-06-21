use std::{pin::Pin, sync::Arc};

use eyre::Context;
use tokio::runtime::Handle;

use crate::prelude::*;

pub struct Update<T: 'static, M: Send + Sync + 'static> {
    handle: Box<dyn Fn(&mut T, M) -> Result<Tasks<M>>>,
}

impl<T: 'static, M: Send + Sync + 'static> Update<T, M> {
    pub fn new(handle: impl Fn(&mut T, M) -> Result<Tasks<M>> + 'static) -> Self {
        Update {
            handle: Box::new(handle),
        }
    }

    pub fn call(&self, state: &mut T, message: M) -> Result<Tasks<M>> {
        (self.handle)(state, message)
    }

    pub fn update(&mut self, handle: &mut T, message: M) -> Result<Tasks<M>> {
        self.call(handle, message)
    }

    pub fn execute(
        &mut self,
        handle: &mut T,
        queue: MessageQueueSender<M>,
        message: M,
        runtime: &Handle,
    ) -> Result<()> {
        self.update(handle, message)?.execute(queue, runtime)
    }
}

pub struct Tasks<M: Send + Sync + 'static> {
    handle: Vec<Pin<Box<dyn Future<Output = Result<M>> + Send + 'static>>>,

    label: Option<String>,
    on_failure: Option<Arc<dyn Fn(Report) -> M + Send + Sync + 'static>>,
}

impl<M: Send + Sync + 'static> Tasks<M> {
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn on_failure(mut self, on_failure: impl Fn(Report) -> M + Send + Sync + 'static) -> Self {
        self.on_failure = Some(Arc::new(on_failure));
        self
    }

    pub fn none() -> Self {
        Tasks {
            handle: Vec::new(),
            label: None,
            on_failure: None,
        }
    }

    pub fn new(task: Vec<impl Future<Output = Result<M>> + Send + 'static>) -> Self {
        Tasks {
            handle: task
                .into_iter()
                .map(|t| Box::pin(t) as Pin<Box<dyn Future<Output = Result<M>> + Send>>)
                .collect(),
            label: None,
            on_failure: None,
        }
    }

    pub fn single(task: impl Future<Output = Result<M>> + Send + 'static) -> Self {
        Tasks {
            handle: vec![Box::pin(task)],
            label: None,
            on_failure: None,
        }
    }

    pub fn execute(self, sender: MessageQueueSender<M>, runtime: &Handle) -> Result<()> {
        for task in self.handle {
            let sender_clone = sender.clone();
            let on_failure_clone = self.on_failure.clone();
            let label_clone = self.label.clone();
            runtime.spawn(async move {
                if let Err(report) = task
                    .await
                    .and_then(|message| sender_clone.send(message).map_err(Report::msg))
                    .wrap_err(if let Some(label) = &label_clone {
                        format!("Task '{}' failed", label)
                    } else {
                        "Task failed".to_string()
                    })
                {
                    if let Some(on_failure) = on_failure_clone {
                        let message = on_failure(report);

                        if let Err(report) = sender_clone.send(message) {
                            if let Some(label) = label_clone {
                                eprintln!("Task '{}' failed: {:?}", label, report);
                            } else {
                                eprintln!("Task failed: {:?}", report);
                            }
                        }
                    } else {
                        eprintln!("Task failed: {:?}", report);
                    }
                }
            });
        }

        Ok(())
    }
}
