use crate::prelude::*;

use smithay_client_toolkit::{
    delegate_pointer,
    reexports::client::{Connection, QueueHandle, protocol::wl_pointer::WlPointer},
    seat::pointer::{PointerEvent, PointerHandler},
};

delegate_pointer!(State);

impl PointerHandler for State {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _pointer: &WlPointer,
        _events: &[PointerEvent],
    ) {
    }
}
