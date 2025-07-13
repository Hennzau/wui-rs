use std::time::Duration;

use smithay_client_toolkit::{
    compositor::CompositorState,
    delegate_registry,
    output::OutputState,
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::SeatState,
    shell::{
        WaylandSurface,
        wlr_layer::{LayerShell, LayerSurface},
        xdg::{XdgShell, window::Window},
    },
};
use tokio::task::JoinHandle;
use wayland_client::{
    Connection, EventQueue, QueueHandle,
    globals::registry_queue_init,
    protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer},
};

use crate::prelude::*;

pub(crate) mod compositor;
pub(crate) mod keyboard;
pub(crate) mod layer;
pub(crate) mod output;
pub(crate) mod pointer;
pub(crate) mod seat;
pub(crate) mod window;

pub struct WaylandBackend<Message: 'static + Send + Sync> {
    pub(crate) state: State<Message>,
    pub(crate) event_queue: EventQueue<State<Message>>,
}

impl<Message: 'static + Send + Sync> WaylandBackend<Message> {
    pub(crate) fn new(client: Client<Message>) -> Result<(Self, WaylandProtocol<Message>)> {
        let (state, protocol, event_queue) = State::new(client)?;

        Ok((Self { state, event_queue }, protocol))
    }

    pub(crate) async fn run(mut self) -> Result<()> {
        let event_loop: JoinHandle<Result<()>> = tokio::task::spawn(async move {
            loop {
                self.event_queue.flush()?;

                if let Some(guard) = self.event_queue.prepare_read() {
                    if let Err(e) = guard.read_without_dispatch() {
                        eprintln!("Error reading events: {:?}", e);
                    }
                }

                self.event_queue.dispatch_pending(&mut self.state).unwrap();

                tokio::time::sleep(Duration::from_millis(16)).await;
            }
        });

        event_loop.await?
    }
}

pub(crate) struct WaylandProtocol<Message: 'static + Send + Sync> {
    pub(crate) connection: Connection,
    pub(crate) queue_handle: QueueHandle<State<Message>>,
    pub(crate) compositor_state: CompositorState,
    pub(crate) xdg_shell: XdgShell,
    pub(crate) layer_shell: LayerShell,
}

impl<Message: 'static + Send + Sync> WaylandProtocol<Message> {
    pub(crate) fn create_layer(&self, view: &View<Message>) -> LayerSurface {
        let wl_surface = self.compositor_state.create_surface(&self.queue_handle);

        let layer = self.layer_shell.create_layer_surface(
            &self.queue_handle,
            wl_surface,
            view.layer,
            Some(view.label.clone()),
            None,
        );

        layer.set_anchor(view.anchor);
        layer.set_keyboard_interactivity(view.keyboard_interactivity);
        layer.set_size(view.size.0, view.size.1);
        layer.set_exclusive_zone(view.exclusive_zone);
        layer.set_margin(view.margin.0, view.margin.1, view.margin.2, view.margin.3);

        layer.commit();

        layer
    }

    pub(crate) fn create_window(&self, view: &View<Message>) -> Window {
        let wl_surface = self.compositor_state.create_surface(&self.queue_handle);

        let window = self
            .xdg_shell
            .create_window(wl_surface, view.decorations, &self.queue_handle);

        window.set_title(&view.label);
        window.set_app_id(&view.label);
        window.set_min_size(view.min_size);
        window.set_max_size(view.max_size);

        window.commit();

        window
    }
}

pub(crate) struct State<Message: 'static + Send + Sync> {
    pub(crate) registry_state: RegistryState,
    pub(crate) seat_state: SeatState,
    pub(crate) output_state: OutputState,

    pub(crate) keyboard: Option<WlKeyboard>,
    pub(crate) pointer: Option<WlPointer>,

    pub(crate) client: Client<Message>,
}

impl<Message: 'static + Send + Sync> State<Message> {
    pub(crate) fn new(
        client: Client<Message>,
    ) -> Result<(Self, WaylandProtocol<Message>, EventQueue<Self>)> {
        let connection = Connection::connect_to_env()?;

        let (globals, event_queue) = registry_queue_init::<Self>(&connection)?;
        let qh = event_queue.handle();

        let protocol = WaylandProtocol {
            connection: connection.clone(),
            queue_handle: qh.clone(),
            compositor_state: CompositorState::bind(&globals, &qh)?,
            xdg_shell: XdgShell::bind(&globals, &qh)?,
            layer_shell: LayerShell::bind(&globals, &qh)?,
        };

        Ok((
            Self {
                registry_state: RegistryState::new(&globals),
                seat_state: SeatState::new(&globals, &qh),
                output_state: OutputState::new(&globals, &qh),

                keyboard: None,
                pointer: None,

                client,
            },
            protocol,
            event_queue,
        ))
    }
}

delegate_registry!(@<Message: 'static + Send + Sync> State<Message>);

impl<Message: 'static + Send + Sync> ProvidesRegistryState for State<Message> {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
