use crate::prelude::*;

mod attach_child;
mod close_view;
mod create_view;
mod forward_event;

impl OrchestratorInner {
    pub(crate) async fn handle_query(&mut self, query: Query) -> Result<()> {
        let request = query.request;
        let response = query.response;

        match request {
            Request::CreateViewLayer(configuration) => {
                self.handle_create_view_layer(configuration, response)
                    .await?;
            }
            Request::CreateViewWindow(configuration) => {
                self.handle_create_view_window(configuration, response)
                    .await?;
            }
            Request::ForwardEvent { event, id } => {
                self.handle_forward_event(event, id, response).await?;
            }
            Request::CloseView(id) => {
                self.handle_close_view(id, response)?;
            }
            Request::AttachChild { id, child } => {
                self.handle_attach_child(id, child, response)?;
            }
            _ => {
                if let Some(response) = response {
                    response
                        .send(Response::NotImplemented)
                        .map_err(|e| eyre::eyre!("Failed to send response: {:?}", e))?;
                }
            }
        };

        Ok(())
    }
}
