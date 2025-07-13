use smithay_client_toolkit::shell::WaylandSurface;
use tokio::sync::oneshot::Sender;
use wayland_backend::client::ObjectId;

use crate::prelude::*;

pub(crate) async fn handle_close<Message: 'static + Send + Sync>(
    id: ObjectId,
    views: &mut Views<Message>,
    response: Option<Sender<Response>>,
) -> Result<()> {
    match views.remove_by_id(id.clone()) {
        Some(view) => {
            if let Some(layer) = view.layer_surface {
                layer.wl_surface().destroy();
            }

            if let Some(window) = view.window_surface {
                window.wl_surface().destroy();
            }

            send_response::<Message>(response, Response::Success)
        }
        None => send_response::<Message>(
            response,
            Response::Failed(format!("View with id {} not found", id)),
        ),
    }
}
