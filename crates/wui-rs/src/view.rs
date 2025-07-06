use smithay_client_toolkit::{
    seat::{
        keyboard::{KeyEvent, Keysym, Modifiers},
        pointer::PointerEvent,
    },
    shell::{
        wlr_layer::{LayerSurface, LayerSurfaceConfigure},
        xdg::window::{Window, WindowConfigure},
    },
};
use wayland_backend::client::ObjectId;
use wayland_client::protocol::wl_output::Transform;

pub mod config;
pub use config::*;

use ::wgpu::{Adapter, Device, Queue, Surface};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub(crate) enum ViewEvent {
    ScaleFactorChanged(i32),
    TransformChanged(Transform),
    Frame(u32),
    KeyboardEnter(Vec<Keysym>),
    KeyboardLeave,
    KeyPressed(KeyEvent),
    KeyReleased(KeyEvent),
    KeyModifiersChanged(Modifiers),
    Configure(ViewConfigure),
    Closed,
    PointerFrame(PointerEvent),
}

pub(crate) enum ViewHandle {
    LayerSurface(LayerSurface),
    Window(Window),
}

pub struct View {
    pub(crate) id: ObjectId,

    pub(crate) handle: ViewHandle,
    pub(crate) surface: Surface<'static>,
    pub(crate) adapter: Adapter,
    pub(crate) device: Device,
    pub(crate) queue: Queue,

    pub(crate) client: Client,
}

impl View {
    pub fn id(&self) -> ObjectId {
        self.id.clone()
    }
}
