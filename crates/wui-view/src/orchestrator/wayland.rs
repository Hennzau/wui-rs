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

pub struct WaylandBackend {
    pub(crate) state: State,
    pub(crate) event_queue: EventQueue<State>,
}

impl WaylandBackend {
    pub(crate) fn new(client: Client) -> Result<(Self, WaylandProtocol)> {
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

pub(crate) struct WaylandProtocol {
    pub(crate) connection: Connection,
    pub(crate) queue_handle: QueueHandle<State>,
    pub(crate) compositor_state: CompositorState,
    pub(crate) xdg_shell: XdgShell,
    pub(crate) layer_shell: LayerShell,
}

impl WaylandProtocol {
    pub(crate) fn create_layer(&self, configuration: ViewConfiguration) -> LayerSurface {
        let wl_surface = self.compositor_state.create_surface(&self.queue_handle);

        let layer = self.layer_shell.create_layer_surface(
            &self.queue_handle,
            wl_surface,
            configuration.layer,
            Some(configuration.namespace),
            None,
        );

        layer.set_anchor(configuration.anchor);
        layer.set_keyboard_interactivity(configuration.keyboard_interactivity);
        layer.set_size(configuration.size.0, configuration.size.1);
        layer.set_exclusive_zone(configuration.exclusive_zone);
        layer.set_margin(
            configuration.margin.0,
            configuration.margin.1,
            configuration.margin.2,
            configuration.margin.3,
        );

        layer.commit();

        layer
    }

    pub(crate) fn create_window(&self, configuration: ViewConfiguration) -> Window {
        let wl_surface = self.compositor_state.create_surface(&self.queue_handle);

        let window =
            self.xdg_shell
                .create_window(wl_surface, configuration.decorations, &self.queue_handle);

        window.set_title(&configuration.title);
        window.set_app_id(&configuration.app_id);
        window.set_min_size(configuration.min_size);
        window.set_max_size(configuration.max_size);

        window.commit();

        window
    }
}

pub(crate) struct State {
    pub(crate) registry_state: RegistryState,
    pub(crate) seat_state: SeatState,
    pub(crate) output_state: OutputState,

    pub(crate) keyboard: Option<WlKeyboard>,
    pub(crate) pointer: Option<WlPointer>,

    pub(crate) client: Client,
}

impl State {
    pub(crate) fn new(client: Client) -> Result<(Self, WaylandProtocol, EventQueue<Self>)> {
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

delegate_registry!(State);

impl ProvidesRegistryState for State {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
