use tokio::task::JoinHandle;

use crate::prelude::*;

pub struct Application<State: 'static> {
    pub(crate) state: State,
    pub(crate) orchestrator: Orchestrator,
    pub(crate) client: Client,

    pub(crate) views: Box<dyn Fn(&State) -> Vec<Box<dyn ElementBuilder>>>,
}

impl<State: 'static> Application<State> {
    pub fn new(
        state: State,
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
            views: Box::new(views),
        })
    }

    pub async fn run(self) -> Result<()> {
        let orchestrator: JoinHandle<Result<()>> =
            tokio::task::spawn(async move { self.orchestrator.run().await });

        self.client.query(Request::StartCachingViews).await?;

        for view in (self.views)(&self.state) {
            let _ = view.build(self.client.clone()).await??;
        }

        self.client.query(Request::GarbageViews).await?;

        println!("Press Ctrl+C to terminate the application.");

        tokio::select! {
            result = orchestrator => {
                result?
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Received Ctrl+C, shutting down the application.");
                Ok(())
            }
        }
    }
}
