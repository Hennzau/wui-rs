use std::sync::Arc;

use eyre::OptionExt;
use smithay_client_toolkit::{
    seat::{
        keyboard::{KeyEvent, Keysym, Modifiers},
        pointer::PointerEvent,
    },
    shell::{WaylandSurface, wlr_layer::LayerSurface, xdg::window::Window},
};
use tokio::task::JoinHandle;
use wayland_backend::client::ObjectId;
use wayland_client::protocol::wl_output::Transform;

pub mod config;
pub use config::*;

use ::wgpu::{Adapter, Device, Queue, Surface};

use crate::{element::Element, prelude::*};

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

    pub(crate) handle: ViewHandle,

    pub(crate) surface: Arc<Surface<'static>>,
    pub(crate) adapter: Arc<Adapter>,
    pub(crate) device: Arc<Device>,
    pub(crate) queue: Arc<Queue>,

    pub(crate) child: Option<Box<dyn Element>>,
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

    pub(crate) fn set_child(&mut self, child: Box<dyn Element>) -> Result<()> {
        if self.child.is_none() {
            self.child.replace(child);

            Ok(())
        } else {
            Err(Report::msg("This view already has a child"))
        }
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

    pub fn id(&self) -> ObjectId {
        self.id.clone()
    }
}

impl Element for View {}

pub struct ViewBuilder {
    pub(crate) kind: ViewKind,
    pub(crate) configuration: ViewConfiguration,
    pub(crate) child: Option<Box<dyn ElementBuilder>>,
}

impl Default for ViewBuilder {
    fn default() -> Self {
        Self {
            kind: ViewKind::Layer,
            configuration: ViewConfiguration::default(),
            child: None,
        }
    }
}

impl ViewBuilder {
    pub fn with_kind(self, kind: ViewKind) -> Self {
        Self {
            kind: kind,
            configuration: self.configuration,
            child: self.child,
        }
    }

    pub fn with_configuration(self, configuration: ViewConfiguration) -> Self {
        Self {
            kind: self.kind,
            configuration: configuration,
            child: self.child,
        }
    }

    pub fn with_child(self, child: Box<dyn ElementBuilder>) -> Self {
        Self {
            kind: self.kind,
            configuration: self.configuration,
            child: Some(child),
        }
    }
}

impl ElementBuilder for ViewBuilder {
    fn build(self: Box<Self>, client: Client) -> JoinHandle<Result<ElementHandle>> {
        tokio::task::spawn(async move {
            let child = self.child.ok_or_eyre("A view must have a child")?;
            let child = child.build(client.clone()).await??;

            match child {
                ElementHandle::SelfContained(child) => {
                    let request = match self.kind {
                        ViewKind::Layer => Request::CreateViewLayer(self.configuration),
                        ViewKind::Window => Request::CreateViewWindow(self.configuration),
                    };

                    let view = match client.query(request).await? {
                        Response::Success(id) => id.ok_or_eyre("Could not create a view")?,
                        _ => eyre::bail!("This kind of response is not handled"),
                    };

                    client.send(Request::AttachChild {
                        id: view,
                        child: child,
                    })?;

                    Ok(ElementHandle::External)
                }
                _ => Err(Report::msg("A view cannot have a view as unique child!")),
            }
        })
    }
}
