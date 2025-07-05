use smithay_client_toolkit::{
    seat::{
        keyboard::{KeyEvent, Keysym, Modifiers},
        pointer::PointerEvent,
    },
    shell::{
        WaylandSurface,
        wlr_layer::{Anchor, KeyboardInteractivity, Layer, LayerSurface, LayerSurfaceConfigure},
        xdg::window::{Window, WindowConfigure, WindowDecorations},
    },
};
use wayland_backend::client::ObjectId;
use wayland_client::{Proxy, protocol::wl_output::Transform};
use wgpu::{Adapter, Device, Queue, Surface};

use crate::prelude::*;

mod configure;

pub struct ViewConfiguration {
    pub namespace: Option<String>,
    pub layer: Layer,
    pub anchor: Anchor,
    pub keyboard_interactivity: KeyboardInteractivity,
    pub size: (u32, u32),
    pub min_size: Option<(u32, u32)>,
    pub max_size: Option<(u32, u32)>,
    pub margin: (i32, i32, i32, i32),
    pub exclusive_zone: i32,
    pub decorations: WindowDecorations,
    pub title: String,
    pub app_id: String,
}

impl Default for ViewConfiguration {
    fn default() -> Self {
        Self {
            namespace: None,
            layer: Layer::Top,
            anchor: Anchor::TOP,
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            size: (0, 0),
            min_size: None,
            max_size: None,
            margin: (0, 0, 0, 0),
            exclusive_zone: 0,
            decorations: WindowDecorations::ServerDefault,
            title: String::from("wui_rs.default"),
            app_id: String::from("io.github.wui_rs.default"),
        }
    }
}

pub enum ViewHandle {
    LayerSurface(LayerSurface),
    Window(Window),
}

impl ViewHandle {
    pub fn id(&self) -> ObjectId {
        match self {
            ViewHandle::LayerSurface(layer_surface) => layer_surface.wl_surface().id(),
            ViewHandle::Window(window) => window.wl_surface().id(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WaylandViewConfigure {
    LayerSurface(LayerSurfaceConfigure),
    Window(WindowConfigure),
}

#[derive(Debug, Clone)]
pub enum ViewEvent {
    ScaleFactorChanged(i32),
    TransformChanged(Transform),
    Frame(u32),
    KeyboardEnter(Vec<Keysym>),
    KeyboardLeave,
    KeyPressed(KeyEvent),
    KeyReleased(KeyEvent),
    KeyModifiersChanged(Modifiers),
    Configure(WaylandViewConfigure),
    Closed,
    PointerFrame(PointerEvent),
}

pub struct View {
    surface: Surface<'static>,
    adapter: Adapter,
    handle: ViewHandle,
    device: Device,
    queue: Queue,

    configuration: ViewConfiguration,
}

impl View {
    pub fn new(
        handle: ViewHandle,
        surface: Surface<'static>,
        adapter: Adapter,
        device: Device,
        queue: Queue,
        configuration: ViewConfiguration,
    ) -> Self {
        Self {
            handle,
            surface,
            adapter,
            device,
            queue,
            configuration,
        }
    }

    pub async fn update(&mut self, event: ViewEvent) -> Result<()> {
        match event {
            ViewEvent::Configure(configure) => self.configure(configure),
            ViewEvent::KeyboardEnter(_) => println!("Keyboard entered view: {}", self.id()),
            ViewEvent::Closed => {
                println!("View closed: {}", self.id());
            }
            _ => {}
        }

        Ok(())
    }

    pub fn id(&self) -> ObjectId {
        self.handle.id()
    }

    pub fn surface(&self) -> &Surface<'static> {
        &self.surface
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn configuration(&self) -> &ViewConfiguration {
        &self.configuration
    }
}
