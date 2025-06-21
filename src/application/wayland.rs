use crate::prelude::*;

mod compositor;
mod keyboard;
mod output;
mod pointer;
mod seat;
mod shell;

use smithay_client_toolkit::{
    compositor::CompositorState,
    delegate_registry,
    output::OutputState,
    reexports::client::{
        Connection, EventQueue,
        globals::registry_queue_init,
        protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer},
    },
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::SeatState,
    shell::wlr_layer::LayerShell,
};

pub struct Wayland {
    registry_state: RegistryState,
    seat_state: SeatState,
    output_state: OutputState,
    compositor_state: CompositorState,
    layer_shell: LayerShell,

    keyboard: Option<WlKeyboard>,
    pointer: Option<WlPointer>,
}

impl Wayland {
    pub fn new() -> Result<(Self, EventQueue<Wayland>)> {
        let conn = Connection::connect_to_env()?;

        let (globals, event_queue) = registry_queue_init::<Wayland>(&conn)?;
        let qh = event_queue.handle();

        Ok((
            Self {
                registry_state: RegistryState::new(&globals),
                seat_state: SeatState::new(&globals, &qh),
                output_state: OutputState::new(&globals, &qh),
                compositor_state: CompositorState::bind(&globals, &qh)
                    .expect("wl_compositor not available"),
                layer_shell: LayerShell::bind(&globals, &qh).expect("layer shell not available"),

                keyboard: None,
                pointer: None,
            },
            event_queue,
        ))
    }

    pub fn dispatch(&mut self, event_queue: &mut EventQueue<Self>) -> Result<()> {
        event_queue
            .dispatch_pending(self)
            .map(|_| ())
            .map_err(Report::msg)
    }
}

delegate_registry!(Wayland);

impl ProvidesRegistryState for Wayland {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
