use smithay_client_toolkit::shell::WaylandSurface;
use wayland_backend::client::ObjectId;
use wayland_client::Proxy;

use crate::prelude::*;

impl OrchestratorInner {
    pub(crate) async fn handle_query(&mut self, query: Query) -> Result<()> {
        let request = query.request;
        let response = query.response;

        match request {
            Request::CreateViewLayer(configuration) => {
                let view_id = self.create_view_layer(configuration).await?;

                match response {
                    Some(response) => response
                        .send(Response::ViewLayer(view_id))
                        .map_err(|e| eyre::eyre!("Failed to send response: {:?}", e)),
                    None => Ok(()),
                }
            }
            Request::CreateViewWindow(configuration) => {
                let view_id = self.create_view_window(configuration).await?;

                match response {
                    Some(response) => response
                        .send(Response::ViewWindow(view_id))
                        .map_err(|e| eyre::eyre!("Failed to send response: {:?}", e)),
                    None => Ok(()),
                }
            }
            Request::ForwardEvent { event, id } => {
                println!("Forwarding event: {:?}", event);

                Ok(())
            }
            _ => match response {
                Some(response) => response
                    .send(Response::NotImplemented)
                    .map_err(|e| eyre::eyre!("Failed to send response: {:?}", e)),
                None => Ok(()),
            },
        }
    }

    async fn create_view_layer(&mut self, configuration: ViewConfiguration) -> Result<ObjectId> {
        let layer = self.protocol.create_layer(configuration);
        let (surface, adapter, device, queue) =
            self.create_wgpu_primitives(layer.wl_surface()).await?;
        let id = layer.wl_surface().id();
        let handle = ViewHandle::LayerSurface(layer);
        let client = self.client.clone();

        let result = id.clone();

        self.views.insert(
            id.clone(),
            View {
                id,
                handle,
                client,
                surface,
                adapter,
                device,
                queue,
            },
        );

        Ok(result)
    }

    async fn create_view_window(&mut self, configuration: ViewConfiguration) -> Result<ObjectId> {
        let window = self.protocol.create_window(configuration);
        let (surface, adapter, device, queue) =
            self.create_wgpu_primitives(window.wl_surface()).await?;
        let id = window.wl_surface().id();
        let handle = ViewHandle::Window(window);
        let client = self.client.clone();

        let result = id.clone();

        self.views.insert(
            id.clone(),
            View {
                id,
                handle,
                client,
                surface,
                adapter,
                device,
                queue,
            },
        );

        Ok(result)
    }
}
