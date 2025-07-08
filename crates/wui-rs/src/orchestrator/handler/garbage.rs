use crate::prelude::*;
use tokio::sync::oneshot::Sender;

impl OrchestratorInner {
    pub(crate) fn handle_garbage(&mut self, response: Option<Sender<Response>>) -> Result<()> {
        let mut result = move || -> Result<()> { self.views.garbage() };

        match result() {
            Ok(_) => {
                if let Some(response) = response {
                    response
                        .send(Response::Success(None))
                        .map_err(|e| eyre::eyre!("Failed to send response: {:?}", e))?;
                }
            }
            Err(e) => {
                if let Some(response) = response {
                    response
                        .send(Response::Failed(e.to_string()))
                        .map_err(|e| eyre::eyre!("Failed to send error response: {:?}", e))?;
                }
            }
        }

        Ok(())
    }
}
