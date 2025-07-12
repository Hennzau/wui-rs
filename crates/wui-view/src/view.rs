use std::sync::Arc;

use smithay_client_toolkit::{
    reexports::client::{backend::ObjectId, protocol::wl_output::Transform},
    seat::{
        keyboard::{KeyEvent, Keysym, Modifiers},
        pointer::PointerEvent,
    },
    shell::{WaylandSurface, wlr_layer::LayerSurface, xdg::window::Window},
};

pub mod config;
pub use config::*;

use ::wgpu::{Adapter, Device, Queue, Surface};

use crate::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum ViewEvent {
    Configure(ViewConfigure),
    ScaleFactorChanged(i32),
    TransformChanged(Transform),
    Frame,
    KeyboardEnter(Vec<Keysym>),
    KeyboardLeave,
    KeyPressed(KeyEvent),
    KeyReleased(KeyEvent),
    KeyModifiersChanged(Modifiers),
    PointerFrame(PointerEvent),
}

pub enum ViewKind {
    Layer,
    Window,
}

pub(crate) enum ViewHandle {
    LayerSurface(LayerSurface),
    Window(Window),
}

pub struct View {
    pub(crate) id: ObjectId,
    pub(crate) namespace: String,

    pub(crate) handle: ViewHandle,

    pub(crate) surface: Arc<Surface<'static>>,
    pub(crate) adapter: Arc<Adapter>,
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,
}

impl View {
    pub(crate) async fn update(&mut self, event: ViewEvent) -> Result<()> {
        match event {
            ViewEvent::Configure(configure) => {
                self.configure(configure);
            }
            _ => {}
        }

        Ok(())
    }
    pub(crate) fn close(&self) {
        match self.handle {
            ViewHandle::LayerSurface(ref layer_surface) => {
                layer_surface.wl_surface().destroy();
            }
            ViewHandle::Window(ref window) => {
                window.wl_surface().destroy();
            }
        }
    }

    pub fn namespace(&self) -> String {
        self.namespace.clone()
    }

    pub fn id(&self) -> ObjectId {
        self.id.clone()
    }
}
