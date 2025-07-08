use smithay_client_toolkit::shell::WaylandSurface;
use tokio::sync::oneshot::Sender;
use wayland_backend::client::ObjectId;
use wayland_client::Proxy;

use crate::{prelude::OrchestratorInner, view::ViewConfiguration};

use crate::prelude::*;

impl OrchestratorInner {
    async fn create_view_layer(&mut self, configuration: ViewConfiguration) -> Result<ObjectId> {
        let namespace = configuration.namespace.clone();
        if let Some(view) = self.views.get_id(&namespace) {
            return Ok(view.clone());
        }

        let layer = self.protocol.create_layer(configuration);
        let (surface, adapter, device, queue) =
            self.create_wgpu_primitives(layer.wl_surface()).await?;
        let id = layer.wl_surface().id();
        let handle = ViewHandle::LayerSurface(layer);

        let result = id.clone();

        self.views.insert(View {
            id,
            namespace,
            handle,
            surface,
            adapter,
            device,
            queue,
            child: None,
        });

        Ok(result)
    }

    async fn create_view_window(&mut self, configuration: ViewConfiguration) -> Result<ObjectId> {
        let namespace = configuration.namespace.clone();
        if let Some(view) = self.views.get_id(&namespace) {
            return Ok(view.clone());
        }

        let window = self.protocol.create_window(configuration);
        let (surface, adapter, device, queue) =
            self.create_wgpu_primitives(window.wl_surface()).await?;
        let id = window.wl_surface().id();
        let handle = ViewHandle::Window(window);

        let result = id.clone();

        self.views.insert(View {
            id,
            namespace,
            handle,
            surface,
            adapter,
            device,
            queue,
            child: None,
        });

        Ok(result)
    }

    pub(crate) async fn handle_create_view_layer(
        &mut self,
        configuration: ViewConfiguration,
        response: Option<Sender<Response>>,
    ) -> Result<()> {
        match self.create_view_layer(configuration).await {
            Ok(view_id) => {
                if let Some(response) = response {
                    response
                        .send(Response::Success(Some(view_id)))
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

    pub(crate) async fn handle_create_view_window(
        &mut self,
        configuration: ViewConfiguration,
        response: Option<Sender<Response>>,
    ) -> Result<()> {
        match self.create_view_window(configuration).await {
            Ok(view_id) => {
                if let Some(response) = response {
                    response
                        .send(Response::Success(Some(view_id)))
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
