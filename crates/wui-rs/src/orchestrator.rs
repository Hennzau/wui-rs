pub mod compositor;
pub mod keyboard;
pub mod layer;
pub mod output;
pub mod pointer;
pub mod seat;
pub mod window;

use crate::prelude::*;

use std::{collections::HashMap, ptr::NonNull, time::Duration};

use raw_window_handle::{
    RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
};
use smithay_client_toolkit::{
    compositor::CompositorState,
    delegate_registry,
    output::OutputState,
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::SeatState,
    shell::{WaylandSurface, wlr_layer::LayerShell, xdg::XdgShell},
};

use wayland_backend::client::ObjectId;
use wayland_client::{
    Connection, EventQueue, Proxy, QueueHandle,
    globals::{GlobalList, registry_queue_init},
    protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer},
};
use wgpu::Instance;

pub use smithay_client_toolkit::shell::wlr_layer::{Anchor, KeyboardInteractivity, Layer};

pub struct Orchestrator {
    state: State,
    event_queue: EventQueue<State>,

    instance: Instance,

    views: HashMap<ObjectId, View>,
}

impl Orchestrator {
    pub fn new() -> Result<Self> {
        let conn = Connection::connect_to_env()?;

        let (globals, event_queue) = registry_queue_init::<State>(&conn)?;
        let qh = event_queue.handle();
        let state = State::new(conn.clone(), globals, &qh)?;

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        Ok(Self {
            state,
            event_queue,

            instance,

            views: HashMap::new(),
        })
    }
    pub async fn create_layer(&mut self, configuration: ViewConfiguration) -> Result<ObjectId> {
        let qh = self.event_queue.handle();

        let wl_surface = self.state.compositor_state.create_surface(&qh);
        let wl_surface_ptr = wl_surface.id().as_ptr();

        let layer = self.state.layer_shell.create_layer_surface(
            &qh,
            wl_surface,
            configuration.layer,
            configuration.namespace.clone(),
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

        let raw_display_handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
            NonNull::new(self.state.conn.backend().display_ptr() as *mut _).unwrap(),
        ));
        let raw_window_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(
            NonNull::new(wl_surface_ptr as *mut _).unwrap(),
        ));

        let surface = unsafe {
            self.instance
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle,
                    raw_window_handle,
                })
                .unwrap()
        };

        let adapter = self
            .instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await?;

        let (device, queue) = adapter.request_device(&Default::default()).await?;

        let view = View::new(
            ViewHandle::LayerSurface(layer),
            surface,
            device,
            queue,
            configuration,
        );
        let view_id = view.id();
        self.views.insert(view.id(), view);

        Ok(view_id)
    }

    pub async fn create_window(&mut self, configuration: ViewConfiguration) -> Result<ObjectId> {
        let qh = self.event_queue.handle();

        let wl_surface = self.state.compositor_state.create_surface(&qh);
        let wl_surface_ptr = wl_surface.id().as_ptr();

        let window = self
            .state
            .xdg_shell
            .create_window(wl_surface, configuration.decorations, &qh);

        window.set_title(&configuration.title);
        window.set_app_id(&configuration.app_id);

        window.commit();

        let raw_display_handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
            NonNull::new(self.state.conn.backend().display_ptr() as *mut _).unwrap(),
        ));
        let raw_window_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(
            NonNull::new(wl_surface_ptr as *mut _).unwrap(),
        ));

        let surface = unsafe {
            self.instance
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle,
                    raw_window_handle,
                })
                .unwrap()
        };

        let adapter = self
            .instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await?;

        let (device, queue) = adapter.request_device(&Default::default()).await?;

        let view = View::new(
            ViewHandle::Window(window),
            surface,
            device,
            queue,
            configuration,
        );
        let view_id = view.id();
        self.views.insert(view.id(), view);

        Ok(view_id)
    }

    pub async fn run(mut self) -> Result<()> {
        loop {
            self.event_queue.dispatch_pending(&mut self.state)?;

            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

pub struct State {
    conn: Connection,
    registry_state: RegistryState,
    seat_state: SeatState,
    output_state: OutputState,
    compositor_state: CompositorState,
    xdg_shell: XdgShell,
    layer_shell: LayerShell,

    keyboard: Option<WlKeyboard>,
    pointer: Option<WlPointer>,
}

impl State {
    pub fn new(conn: Connection, globals: GlobalList, qh: &QueueHandle<Self>) -> Result<Self> {
        Ok(Self {
            conn: conn,
            registry_state: RegistryState::new(&globals),
            seat_state: SeatState::new(&globals, qh),
            output_state: OutputState::new(&globals, qh),
            compositor_state: CompositorState::bind(&globals, qh)?,
            xdg_shell: XdgShell::bind(&globals, qh)?,
            layer_shell: LayerShell::bind(&globals, qh)?,

            keyboard: None,
            pointer: None,
        })
    }
}

delegate_registry!(State);

impl ProvidesRegistryState for State {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
