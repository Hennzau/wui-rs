use tokio::sync::{mpsc::UnboundedSender, oneshot::Sender};
use wgpu::Instance;

use crate::prelude::*;

pub(crate) mod close;
pub(crate) use close::*;

pub(crate) mod create;
pub(crate) use create::*;

pub(crate) mod distribute;
pub(crate) use distribute::*;

pub(crate) fn send_response<Message: 'static + Send + Sync>(
    sender: Option<Sender<Response>>,
    response: Response,
) -> Result<()> {
    if let Some(sender) = sender {
        sender.send(response).map_err(Report::msg)?;
    }
    Ok(())
}

pub(crate) async fn handle_query<Message: 'static + Send + Sync>(
    query: Query<Message>,
    app_views: &mut Views<Message>,
    sender: &UnboundedSender<Message>,
    instance: &Instance,
    protocol: &WaylandProtocol<Message>,
) {
    let request = query.request;
    let response = query.response;

    if let Err(error) = match request {
        Request::Create { views } => {
            handle_create(views, instance, protocol, app_views, response).await
        }
        Request::Close { id } => handle_close(id, app_views, response).await,
        Request::Distribute { id, event } => {
            handle_distribute(id, sender, event, app_views, response).await
        }
        _ => send_response::<Message>(response, Response::Success),
    } {
        eprintln!("Error handling request: {}", error);
    }
}
