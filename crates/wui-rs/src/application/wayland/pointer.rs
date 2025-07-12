use crate::prelude::*;

use smithay_client_toolkit::{
    delegate_pointer,
    reexports::client::{Connection, QueueHandle, protocol::wl_pointer::WlPointer},
    seat::pointer::{PointerEvent, PointerEventKind, PointerHandler},
};
use wayland_client::Proxy;

delegate_pointer!(@<Message: 'static + Send + Sync> State<Message>);

impl<Message: 'static + Send + Sync> PointerHandler for State<Message> {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _pointer: &WlPointer,
        events: &[PointerEvent],
    ) {
        for event in events {
            let position = event.position;
            let id = event.surface.id();

            self.client.send_no_result(Request::Distribute {
                event: {
                    match event.kind {
                        PointerEventKind::Enter { serial: _ } => Event::PointerEnter,
                        PointerEventKind::Leave { serial: _ } => Event::PointerLeave,
                        PointerEventKind::Motion { time: _ } => Event::PointerMoved {
                            x: position.0,
                            y: position.1,
                        },
                        PointerEventKind::Press {
                            time: _,
                            button,
                            serial: _,
                        } => Event::PointerPressed {
                            button: button.into(),
                            x: position.0,
                            y: position.1,
                        },
                        PointerEventKind::Release {
                            time: _,
                            button,
                            serial: _,
                        } => Event::PointerReleased {
                            button: button.into(),
                            x: position.0,
                            y: position.1,
                        },
                        PointerEventKind::Axis {
                            time: _,
                            horizontal,
                            vertical,
                            source: _,
                        } => Event::PointerScrolled {
                            x: position.0,
                            y: position.1,
                            delta_x: horizontal.absolute,
                            delta_y: vertical.absolute,
                        },
                    }
                },
                id: Some(id),
            });
        }
    }
}
