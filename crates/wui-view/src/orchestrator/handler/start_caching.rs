use crate::prelude::*;
use tokio::sync::oneshot::Sender;

impl OrchestratorInner {
    pub(crate) fn handle_start_caching(
        &mut self,
        response: Option<Sender<Response>>,
    ) -> Result<()> {
        self.views.activate_cache();

        if let Some(response) = response {
            response
                .send(Response::Success(None))
                .map_err(|e| eyre::eyre!("Failed to send response: {:?}", e))?;
        }

        Ok(())
    }
}
