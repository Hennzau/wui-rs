use std::{collections::VecDeque, pin::Pin, sync::Arc};

use eyre::Result;
use tokio::{runtime::Handle, sync::Mutex};

pub struct Task<T: Send + 'static> {
    pub handle: Option<Pin<Box<dyn Future<Output = Result<T>> + Send + 'static>>>,
}
impl<T: Send + 'static> Task<T> {
    pub fn none() -> Self {
        Task { handle: None }
    }

    pub fn new(task: impl Future<Output = Result<T>> + Send + 'static) -> Self {
        Task {
            handle: Some(Box::pin(task)),
        }
    }

    pub fn execute(
        self,
        runtime: Handle,
        queue: Arc<Mutex<VecDeque<T>>>,
        on_failure: Arc<dyn Fn(String) -> T + Sync + Send>,
    ) {
        match self.handle {
            Some(handle) => {
                let queue_clone = queue.clone();
                runtime.spawn(async move {
                    let result = handle.await;
                    if let Err(e) = result {
                        queue.lock().await.push_back(on_failure(e.to_string()));
                        return;
                    }

                    let result = result.unwrap();

                    let mut q = queue_clone.lock().await;
                    q.push_back(result);
                });
            }
            None => {}
        }
    }
}
