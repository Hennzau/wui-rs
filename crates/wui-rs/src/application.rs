use tokio::task::JoinHandle;

use crate::prelude::*;

pub struct Application<State: 'static, Message: 'static + Send + Sync> {
    pub(crate) state: State,
    pub(crate) orchestrator: Orchestrator,
    pub(crate) client: Client,

    pub(crate) update: Box<dyn Fn(&mut State, Message) -> Option<Task<Message>>>,
    pub(crate) views: Box<dyn Fn(&State) -> Vec<Box<dyn ElementBuilder>>>,
}

impl<State: 'static + Send, Message: 'static + Send + Sync> Application<State, Message> {
    pub fn new(
        state: State,
        update: impl Fn(&mut State, Message) -> Option<Task<Message>> + 'static,
        views: impl Fn(&State) -> Vec<Box<dyn ElementBuilder>> + 'static,
    ) -> Result<Self>
    where
        Self: Sized + 'static,
    {
        let (orchestrator, client) = Orchestrator::new()?;

        Ok(Application {
            state,
            orchestrator,
            client,
            update: Box::new(update),
            views: Box::new(views),
        })
    }

    pub async fn run(mut self) -> Result<()> {
        let orchestrator: JoinHandle<Result<()>> =
            tokio::task::spawn(async move { self.orchestrator.run().await });

        let (sender, mut receiver) = tokio::sync::mpsc::channel::<Message>(100);
        let (stop_send, mut stop) = tokio::sync::mpsc::channel::<()>(1);

        self.client.query(Request::Caching).await?;

        for view in (self.views)(&self.state) {
            let _ = view.build(self.client.clone()).await??;
        }

        self.client.query(Request::Garbage).await?;

        let runtime = tokio::runtime::Handle::current();
        let mut main = async move || -> Result<()> {
            while let Some(message) = receiver.recv().await {
                if let Some(task) = (self.update)(&mut self.state, message) {
                    task.execute(sender.clone(), &runtime, stop_send.clone())
                        .await;
                }

                self.client.query(Request::Caching).await?;

                for view in (self.views)(&self.state) {
                    let _ = view.build(self.client.clone()).await??;
                }

                self.client.query(Request::Garbage).await?;
            }

            Ok(())
        };

        println!("Press Ctrl+C to terminate the application.");

        tokio::select! {
            result = orchestrator => {
                result?
            }
            result = main() => {
                result
            }
            _ = stop.recv() => {
                println!("Received stop signal, shutting down the application.");
                Ok(())
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Received Ctrl+C, shutting down the application.");
                Ok(())
            }
        }
    }
}

pub type MessageQueueSender<Message> = tokio::sync::mpsc::Sender<Message>;
pub type MessageQueueReceiver<Message> = tokio::sync::mpsc::Receiver<Message>;
