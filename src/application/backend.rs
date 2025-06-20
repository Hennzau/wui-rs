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

mod handler;

pub struct Backend {
    pub registry_state: RegistryState,
    pub seat_state: SeatState,
    pub output_state: OutputState,
    pub compositor_state: CompositorState,
    pub layer_shell: LayerShell,

    pub keyboard: Option<WlKeyboard>,
    pub pointer: Option<WlPointer>,
}

impl Backend {
    pub fn new_with_event_queue() -> (Self, EventQueue<Self>) {
        let conn = Connection::connect_to_env().unwrap();

        let (globals, event_queue) = registry_queue_init::<Backend>(&conn).unwrap();
        let qh = event_queue.handle();

        let backend = Self {
            registry_state: RegistryState::new(&globals),
            seat_state: SeatState::new(&globals, &qh),
            output_state: OutputState::new(&globals, &qh),
            compositor_state: CompositorState::bind(&globals, &qh)
                .expect("wl_compositor not available"),
            layer_shell: LayerShell::bind(&globals, &qh).expect("layer shell not available"),
            keyboard: None,
            pointer: None,
        };

        (backend, event_queue)
    }
}

delegate_registry!(Backend);

impl ProvidesRegistryState for Backend {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
