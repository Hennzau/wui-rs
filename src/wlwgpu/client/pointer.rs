use crate::*;

use smithay_client_toolkit::{
    delegate_pointer,
    reexports::client::{Connection, Proxy, QueueHandle, protocol::wl_pointer::WlPointer},
    seat::pointer::{PointerEvent, PointerEventKind, PointerHandler},
};

delegate_pointer!(Client);

impl PointerHandler for Client {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _pointer: &WlPointer,
        events: &[PointerEvent],
    ) {
        for event in events {
            let (x, y) = event.position;

            self.handle(
                Some(event.surface.id().into()),
                match event.kind {
                    PointerEventKind::Enter { serial: _ } => EventKind::PointerEntered,
                    PointerEventKind::Leave { serial: _ } => EventKind::PointerLeaved,
                    PointerEventKind::Motion { time: _ } => EventKind::PointerMoved { x, y },
                    PointerEventKind::Press {
                        time: _,
                        button,
                        serial: _,
                    } => EventKind::PointerPressed { x, y, button },
                    PointerEventKind::Release {
                        time: _,
                        button,
                        serial: _,
                    } => EventKind::PointerReleased { x, y, button },
                    PointerEventKind::Axis {
                        time: _,
                        horizontal,
                        vertical,
                        source: _,
                    } => EventKind::PointerScrolled {
                        x,
                        y,
                        delta_x: horizontal.absolute,
                        delta_y: vertical.absolute,
                    },
                },
            );
        }
    }
}
