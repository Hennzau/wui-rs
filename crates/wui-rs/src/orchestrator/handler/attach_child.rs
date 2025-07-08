use tokio::sync::oneshot::Sender;
use wayland_backend::client::ObjectId;

use crate::prelude::*;

impl OrchestratorInner {
    pub(crate) fn handle_attach_child(
        &mut self,
        id: ObjectId,
        child: Box<dyn Element>,
        response: Option<Sender<Response>>,
    ) -> Result<()> {
        let result = move || -> Result<()> {
            if let Some(view) = self.views.get_mut(&id) {
                view.set_child(child)?;

                Ok(())
            } else {
                Err(eyre::eyre!("View with id {} not found", id))
            }
        };

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
