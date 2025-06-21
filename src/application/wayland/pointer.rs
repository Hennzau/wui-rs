use crate::application::wayland::Wayland;

use smithay_client_toolkit::{
    delegate_pointer,
    reexports::client::{Connection, QueueHandle, protocol::wl_pointer::WlPointer},
    seat::pointer::{PointerEvent, PointerHandler},
};

delegate_pointer!(Wayland);

impl PointerHandler for Wayland {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _pointer: &WlPointer,
        _events: &[PointerEvent],
    ) {
    }
}
