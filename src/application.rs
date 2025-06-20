use std::{collections::VecDeque, sync::Arc};

use eyre::Result;
use smithay_client_toolkit::reexports::client::EventQueue;
use tokio::sync::Mutex;

use crate::{application::backend::Backend, prelude::*};

mod backend;
pub mod view;

pub struct Application<T, M: Send + Sync + 'static> {
    pub handle: T,
    pub update: Box<dyn Fn(&mut T, M) -> Result<Task<M>>>,
    pub views: Vec<View<T, M>>,

    pub queue: Arc<Mutex<VecDeque<M>>>,
    init_message: Option<M>,

    backend: (Backend, EventQueue<Backend>),
}

pub fn application<T: 'static, M: Send + Sync + 'static>(
    handle: impl Fn() -> T + 'static,
    update: impl Fn(&mut T, M) -> Result<Task<M>> + 'static,
    views: Vec<impl Fn(&T) -> Element<M> + 'static>,
) -> Application<T, M> {
    Application {
        handle: handle(),
        update: Box::new(update),
        views: views.into_iter().map(|view| View::new(view)).collect(),
        queue: Arc::new(Mutex::new(VecDeque::new())),
        init_message: None,
        backend: Backend::new_with_event_queue(),
    }
}

impl<T, M: Send + Sync + 'static> Application<T, M> {
    pub fn with_init_message(mut self, init_message: M) -> Application<T, M> {
        self.init_message = Some(init_message);
        self
    }

    pub fn run(
        mut self,
        on_failure: impl Fn(String) -> M + 'static + Send + Sync,
    ) -> eyre::Result<()> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        let handle = runtime.handle();

        let on_failure: Arc<dyn Fn(String) -> M + Send + Sync> = Arc::new(on_failure);

        let main = async move || -> Result<()> {
            if let Some(init_message) = self.init_message {
                self.queue.lock().await.push_back(init_message);
            }

            let (mut backend, mut event_queue) = self.backend;

            loop {
                tokio::time::sleep(std::time::Duration::from_millis(32)).await;

                event_queue
                    .dispatch_pending(&mut backend)
                    .map_err(eyre::Report::msg)?;

                let mut queue = self.queue.lock().await;

                while let Some(message) = queue.pop_front() {
                    let task = (self.update)(&mut self.handle, message)?;

                    task.execute(handle.clone(), self.queue.clone(), on_failure.clone());
                }
            }
        };

        let ctrlc = tokio::signal::ctrl_c();

        handle.block_on(async move {
            tokio::select! {
                result = main() => result,
                result = ctrlc => {
                    println!("Ctrl-C received, exiting...");
                    result.map_err(eyre::Report::msg)
                }
            }
        })
    }
}
