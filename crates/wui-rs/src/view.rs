use smithay_client_toolkit::shell::{
    WaylandSurface,
    wlr_layer::{Anchor, KeyboardInteractivity, Layer, LayerSurface},
    xdg::window::{Window, WindowDecorations},
};
use wayland_backend::client::ObjectId;
use wayland_client::Proxy;
use wgpu::{Device, Queue, Surface};

pub struct ViewConfiguration {
    pub namespace: Option<String>,
    pub layer: Layer,
    pub anchor: Anchor,
    pub keyboard_interactivity: KeyboardInteractivity,
    pub size: (u32, u32),
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
            keyboard_interactivity: KeyboardInteractivity::None,
            size: (0, 0),
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

pub struct View {
    surface: Surface<'static>,
    handle: ViewHandle,
    device: Device,
    queue: Queue,

    configuration: ViewConfiguration,
}

impl View {
    pub fn new(
        handle: ViewHandle,
        surface: Surface<'static>,
        device: Device,
        queue: Queue,
        configuration: ViewConfiguration,
    ) -> Self {
        Self {
            handle,
            surface,
            device,
            queue,
            configuration,
        }
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
