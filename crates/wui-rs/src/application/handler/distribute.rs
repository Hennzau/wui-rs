use tokio::sync::{mpsc::UnboundedSender, oneshot::Sender};
use wayland_backend::client::ObjectId;

use crate::prelude::*;

pub(crate) async fn handle_distribute<Message: 'static + Send + Sync>(
    id: Option<ObjectId>,
    sender: &UnboundedSender<Message>,
    event: Event,
    views: &mut Views<Message>,
    _: Option<Sender<Response>>,
) -> Result<()> {
    if let Err(error) = {
        if let Some(id) = id {
            if let Some(view) = views.get_mut_by_id(id.clone()) {
                view.on_event(sender, event)?;
            }
        } else {
            for view in views.iter_mut() {
                view.on_event(sender, event.clone())?;
            }
        }

        Ok::<(), Report>(())
    } {
        eprintln!("Error distributing event: {}", error);
    }

    Ok(())
}
