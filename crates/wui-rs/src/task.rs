use std::pin::Pin;

use tokio::{runtime::Handle, sync::mpsc::Sender};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TaskKind {
    User,

    // Specials
    Exit,
}

pub struct Task<Message: 'static + Send + Sync> {
    pub(crate) kind: TaskKind,

    pub(crate) handle:
        Pin<Box<dyn Future<Output = Result<Option<Message>>> + Sync + Send + 'static>>,
}

impl<Message: 'static + Send + Sync> Task<Message> {
    pub fn user(
        task: impl Future<Output = Result<Option<Message>>> + Sync + Send + 'static,
    ) -> Self {
        Task {
            kind: TaskKind::User,
            handle: Box::pin(task),
        }
    }

    pub fn exit() -> Self {
        Task {
            kind: TaskKind::Exit,
            handle: Box::pin(async { Ok(None) }),
        }
    }

    pub(crate) async fn execute(
        self,
        sender: MessageQueueSender<Message>,
        runtime: &Handle,
        stop: Sender<()>,
    ) {
        match self.kind {
            TaskKind::User => {
                runtime.spawn(async move {
                    let message = self.handle.await;

                    match message {
                        Ok(Some(msg)) => {
                            sender.send(msg).await.ok();
                        }
                        Ok(None) => {}
                        Err(e) => {
                            eprintln!("Error executing task: {:?}", e);
                            stop.send(()).await.expect("Failed to send stop signal");
                        }
                    }
                });
            }
            TaskKind::Exit => {
                stop.send(()).await.expect("Failed to send stop signal");
            }
        }
    }
}
