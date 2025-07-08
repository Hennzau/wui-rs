use crate::prelude::*;

pub struct Application<State: 'static> {
    pub(crate) state: State,
    pub(crate) orchestrator: Orchestrator,
    pub(crate) client: Client,

    pub(crate) element: Box<dyn Fn(&State) -> Box<dyn ElementBuilder>>,
}

impl<State: 'static> Application<State> {
    pub fn new(
        state: State,
        element: impl Fn(&State) -> Box<dyn ElementBuilder> + 'static,
    ) -> Result<Self>
    where
        Self: Sized + 'static,
    {
        let (orchestrator, client) = Orchestrator::new()?;

        Ok(Application {
            state,
            orchestrator,
            client,
            element: Box::new(element),
        })
    }

    pub async fn run(self) -> Result<()> {
        loop {}

        Ok(())
    }
}
