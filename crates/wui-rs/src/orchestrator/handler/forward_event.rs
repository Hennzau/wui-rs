use tokio::sync::oneshot::Sender;
use wayland_backend::client::ObjectId;

use crate::prelude::*;

impl OrchestratorInner {
    pub(crate) async fn handle_forward_event(
        &mut self,
        event: ViewEvent,
        id: Option<ObjectId>,
        response: Option<Sender<Response>>,
    ) -> Result<()> {
        let result = async move || -> Result<()> {
            if let Some(id) = id {
                if let Some(view) = self.views.get_mut(&id) {
                    view.update(event.clone()).await?;
                }
            } else {
                for view in self.views.values_mut() {
                    view.update(event.clone()).await?;
                }
            }

            Ok(())
        };

        match result().await {
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
