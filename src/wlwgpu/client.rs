use smithay_client_toolkit::{
    delegate_registry,
    output::OutputState,
    reexports::client::protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer},
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::SeatState,
};

use crate::*;

mod compositor;
mod keyboard;
mod layer;
mod output;
mod pointer;
mod seat;
mod window;

pub(crate) struct Client {
    pub(crate) shell: Shell,

    pub(crate) event_loop: EventLoop,

    pub(crate) keyboard: Option<WlKeyboard>,
    pub(crate) pointer: Option<WlPointer>,

    pub(crate) registry_state: RegistryState,
    pub(crate) seat_state: SeatState,
    pub(crate) output_state: OutputState,
}

impl Client {
    pub(crate) fn new(shell: Shell, event_loop: EventLoop) -> Self {
        let registry_state = RegistryState::new(&shell.wl.globals);
        let seat_state = SeatState::new(&shell.wl.globals, &shell.wl.qh);
        let output_state = OutputState::new(&shell.wl.globals, &shell.wl.qh);

        let (keyboard, pointer) = (None, None);

        Self {
            output_state,
            seat_state,
            registry_state,

            pointer,
            keyboard,

            event_loop,
            shell,
        }
    }

    pub(crate) fn handle(&mut self, id: Option<SurfaceId>, kind: EventKind) {
        if let Some(id) = &id {
            if !self.shell.exists(id) {
                return;
            }
        }

        if let Err(e) = self.event_loop.call(&mut self.shell, Event { id, kind }) {
            println!("Error {}", e);
        }
    }
}

delegate_registry!(Client);

impl ProvidesRegistryState for Client {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
