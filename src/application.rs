use std::sync::Arc;

use std::borrow::BorrowMut;
use tokio::sync::Mutex;

use crate::prelude::*;

mod wayland;

pub use wayland::Wayland;

pub struct Application<T: 'static, M: Send + Sync + 'static> {
    handle: Arc<Mutex<T>>,
    update: Update<T, M>,
    queue: MessageQueue<M>,

    views: Views<T, M>,
}

pub fn application<T: 'static, M: Send + Sync + 'static>(
    constructor: impl Fn() -> T + 'static,
    update: impl Fn(&mut T, M) -> Result<Tasks<M>> + 'static,
    views: Vec<impl Fn(&T) -> View<M> + 'static>,
) -> Result<Application<T, M>> {
    Ok(Application {
        handle: Arc::new(Mutex::new(constructor())),
        update: Update::new(update),
        queue: MessageQueue::new(),
        views: Views::new(views)?,
    })
}

impl<T: 'static, M: Send + Sync + 'static> Application<T, M> {
    pub fn with_message(self, message: M) -> Result<Self> {
        self.queue().send(message)?;

        Ok(self)
    }

    pub fn queue(&self) -> &MessageQueue<M> {
        &self.queue
    }

    pub fn run(self) -> Result<()> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;

        let handle = runtime.handle();

        let mut queue = self.queue;
        let mut update = self.update;
        let handle_clone = self.handle.clone();

        let views = self.views;
        let sender = views.sender();

        let mut main = async move || -> Result<()> {
            loop {
                while let Some(message) = queue.receive().await {
                    let mut state = handle_clone.lock().await;
                    update.execute(state.borrow_mut(), queue.sender(), message, handle)?;
                    sender.send(Events::StateMightHaveChanged)?;
                }
            }
        };

        let ctrlc = tokio::signal::ctrl_c();

        handle.block_on(async move {
            tokio::select! {
                result = main() => result,
                result = views.run(self.handle.clone()) => result,
                result = ctrlc => {
                    println!("Ctrl-C received, exiting...");
                    result.map_err(eyre::Report::msg)
                }
            }
        })
    }
}
