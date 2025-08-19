use smithay_client_toolkit::{
    delegate_registry,
    output::OutputState,
    reexports::client::{
        QueueHandle,
        protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer},
    },
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

pub(crate) struct Client<Message> {
    pub(crate) msg: Vec<Message>,

    pub(crate) widgets: Widgets<Message>,

    pub(crate) shell: Shell<Message>,

    pub(crate) keyboard: Option<WlKeyboard>,
    pub(crate) pointer: Option<WlPointer>,

    pub(crate) registry_state: RegistryState,
    pub(crate) seat_state: SeatState,
    pub(crate) output_state: OutputState,
}

impl<Message: 'static> Client<Message> {
    pub(crate) fn new(shell: Shell<Message>, qh: &QueueHandle<Self>) -> Self {
        let registry_state = RegistryState::new(&shell.wl.globals);
        let seat_state = SeatState::new(&shell.wl.globals, qh);
        let output_state = OutputState::new(&shell.wl.globals, qh);

        let (keyboard, pointer) = (None, None);

        let widgets = Widgets::new();

        let msg = Vec::new();

        Self {
            output_state,
            seat_state,
            registry_state,

            pointer,
            keyboard,

            shell,

            widgets,

            msg,
        }
    }

    pub(crate) fn handle(&mut self, id: Option<SurfaceId>, kind: EventKind) {
        if let Some(id) = &id
            && !self.shell.exists(id) {
                return;
            }

        if let Err(e) =
            self.widgets
                .handle_event(&mut self.msg, &mut self.shell, Event { id, kind })
        {
            println!("Error {}", e);
        }
    }
}

delegate_registry!(@<Message: 'static> Client<Message>);

impl<Message: 'static> ProvidesRegistryState for Client<Message> {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
