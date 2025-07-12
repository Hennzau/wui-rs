use crate::prelude::*;

use smithay_client_toolkit::{
    delegate_pointer,
    reexports::client::{Connection, QueueHandle, protocol::wl_pointer::WlPointer},
    seat::pointer::{PointerEvent, PointerHandler},
};
use wayland_client::Proxy;

delegate_pointer!(State);

impl PointerHandler for State {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _pointer: &WlPointer,
        events: &[PointerEvent],
    ) {
        for event in events {
            self.client.send_no_result(Request::ForwardEvent {
                event: ViewEvent::PointerFrame(event.clone()),
                id: Some(event.surface.id()),
            });
        }
    }
}
