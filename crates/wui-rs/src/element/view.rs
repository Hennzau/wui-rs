use std::{collections::HashMap, sync::Arc};

use eyre::OptionExt;
use smithay_client_toolkit::{
    seat::keyboard::{Keysym, Modifiers},
    shell::{
        WaylandSurface,
        wlr_layer::LayerSurface,
        xdg::window::{Window, WindowDecorations},
    },
};

pub use smithay_client_toolkit::shell::wlr_layer::{Anchor, KeyboardInteractivity, Layer};

use tokio::sync::mpsc::UnboundedSender;
use wayland_backend::client::ObjectId;
use wayland_client::Proxy;
use wgpu::{Adapter, Device, Instance, Queue, Surface};

use crate::prelude::*;

pub(crate) mod configure;

#[derive(Debug, Clone)]
pub enum Event {
    Configure {
        width: u32,
        height: u32,
    },
    Render,

    KeyboardEnter,
    KeyboardLeave,

    KeyPressed {
        key: Keysym,
    },
    KeyReleased {
        key: Keysym,
    },
    KeyModifiersChanged {
        modifiers: Modifiers,
    },

    PointerEnter,
    PointerLeave,
    PointerMoved {
        x: f64,
        y: f64,
    },

    PointerPressed {
        x: f64,
        y: f64,
        button: u32,
    },
    PointerReleased {
        x: f64,
        y: f64,
        button: u32,
    },
    PointerScrolled {
        x: f64,
        y: f64,
        delta_x: f64,
        delta_y: f64,
    },
}

pub struct View<Message: 'static + Send + Sync> {
    pub(crate) id: Option<ObjectId>,
    pub(crate) child: Option<Element<Message>>,

    pub(crate) layer_surface: Option<LayerSurface>,
    pub(crate) window_surface: Option<Window>,

    pub(crate) surface: Option<Arc<Surface<'static>>>,
    pub(crate) adapter: Option<Arc<Adapter>>,
    pub(crate) device: Option<Arc<Device>>,
    pub(crate) queue: Option<Arc<Queue>>,

    pub(crate) window: bool,
    pub(crate) label: String,
    pub(crate) layer: Layer,
    pub(crate) anchor: Anchor,
    pub(crate) keyboard_interactivity: KeyboardInteractivity,
    pub(crate) size: (u32, u32),
    pub(crate) min_size: Option<(u32, u32)>,
    pub(crate) max_size: Option<(u32, u32)>,
    pub(crate) exclusive_zone: i32,
    pub(crate) margin: (i32, i32, i32, i32),
    pub(crate) decorations: WindowDecorations,
}

impl<Message: 'static + Send + Sync> Widget<Message> for View<Message> {
    fn on_event(&mut self, messages: &UnboundedSender<Message>, event: Event) -> Result<()> {
        match event {
            Event::Render => {
                self.render();
            }
            Event::Configure { width, height } => {
                self.configure(width, height)?;
            }
            event => {
                if let Some(child) = &mut self.child {
                    child.on_event(messages, event)?;
                }
            }
        }

        Ok(())
    }

    fn render(&self) {
        self.child.as_ref().map(|child| {
            child.render();
        });
    }
}

impl<Message: 'static + Send + Sync> Default for View<Message> {
    fn default() -> Self {
        Self {
            id: None,
            child: None,

            layer_surface: None,
            window_surface: None,
            surface: None,
            adapter: None,
            device: None,
            queue: None,

            window: false,
            label: String::default(),
            layer: Layer::Top,
            anchor: Anchor::TOP,
            keyboard_interactivity: KeyboardInteractivity::default(),
            size: (0, 0),
            exclusive_zone: 0,
            margin: (0, 0, 0, 0),
            decorations: WindowDecorations::ServerDefault,
            min_size: None,
            max_size: None,
        }
    }
}

impl<Message: 'static + Send + Sync> View<Message> {
    pub(crate) async fn build(
        mut self: Self,
        instance: &Instance,
        protocol: &WaylandProtocol<Message>,
    ) -> Result<Self> {
        let wl_surface = match self.window {
            true => {
                self.window_surface = Some(protocol.create_window(&self));
                self.window_surface.as_ref().unwrap().wl_surface()
            }
            false => {
                self.layer_surface = Some(protocol.create_layer(&self));
                self.layer_surface.as_ref().unwrap().wl_surface()
            }
        };

        let id = wl_surface.id();
        self.id = Some(id);

        let (surface, adapter, device, queue) =
            create_wgpu_primitives(instance, &protocol.connection, wl_surface).await?;

        self.surface = Some(surface);
        self.adapter = Some(adapter);
        self.device = Some(device);
        self.queue = Some(queue);

        Ok(self)
    }

    pub fn child(self, child: impl Into<Element<Message>>) -> Self {
        Self {
            child: Some(child.into()),
            ..self
        }
    }

    pub fn window(self, window: bool) -> Self {
        Self {
            window: window,
            ..self
        }
    }

    pub fn label(self, label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..self
        }
    }

    pub fn anchor(self, anchor: Anchor) -> Self {
        Self {
            anchor: anchor,
            ..self
        }
    }

    pub fn keyboard_interactivity(self, keyboard_interactivity: KeyboardInteractivity) -> Self {
        Self {
            keyboard_interactivity: keyboard_interactivity,
            ..self
        }
    }

    pub fn size(self, width: u32, height: u32) -> Self {
        Self {
            size: (width, height),
            ..self
        }
    }

    pub fn exclusive_zone(self, exclusive_zone: i32) -> Self {
        Self {
            exclusive_zone: exclusive_zone,
            ..self
        }
    }

    pub fn margin(self, margin: (i32, i32, i32, i32)) -> Self {
        Self {
            margin: margin,
            ..self
        }
    }
}

pub fn view<Message: 'static + Send + Sync>() -> View<Message> {
    View::default()
}

pub struct Views<Message: 'static + Send + Sync> {
    pub(crate) lut: HashMap<ObjectId, String>,
    pub(crate) data: HashMap<String, View<Message>>,
}

impl<Message: 'static + Send + Sync> Views<Message> {
    pub(crate) fn new() -> Self {
        Self {
            lut: HashMap::new(),
            data: HashMap::new(),
        }
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = &mut View<Message>> {
        self.data.values_mut()
    }

    pub(crate) fn get_mut_by_id(&mut self, id: ObjectId) -> Option<&mut View<Message>> {
        if let Some(label) = self.lut.get(&id) {
            self.data.get_mut(label)
        } else {
            None
        }
    }

    pub(crate) fn remove_by_id(&mut self, id: ObjectId) -> Option<View<Message>> {
        match self.label(id) {
            Some(label) => self.remove(&label),
            None => None,
        }
    }

    pub(crate) fn remove(&mut self, label: &str) -> Option<View<Message>> {
        self.data.remove(label)
    }

    pub(crate) fn label(&self, id: ObjectId) -> Option<String> {
        self.lut.get(&id).cloned()
    }

    pub(crate) async fn merge(
        &mut self,
        mut other: Views<Message>,
        instance: &Instance,
        protocol: &WaylandProtocol<Message>,
    ) -> Result<()> {
        let mut built_views = Vec::new();

        let existing_labels: std::collections::HashSet<String> =
            self.data.keys().cloned().collect();
        let other_labels = other.data.keys().cloned().collect::<Vec<_>>();

        for (label, view) in other.data.drain() {
            if !existing_labels.contains(&label) {
                let built = view.build(instance, protocol).await?;
                built_views.push((label, built));
            }
        }

        for (label, view) in built_views {
            self.lut.insert(
                view.id
                    .clone()
                    .ok_or_eyre("View should have an id, there must have been a bug")?,
                label.clone(),
            );
            self.data.insert(label, view);
        }

        self.data.retain(|label, _| other_labels.contains(label));

        Ok(())
    }
}

impl<Message: 'static + Send + Sync> From<View<Message>> for Views<Message> {
    fn from(value: View<Message>) -> Self {
        Self {
            lut: HashMap::new(),
            data: HashMap::from([(value.label.clone(), value)]),
        }
    }
}
impl<Message: 'static + Send + Sync> From<Vec<View<Message>>> for Views<Message> {
    fn from(value: Vec<View<Message>>) -> Self {
        Self {
            lut: HashMap::new(),
            data: value
                .into_iter()
                .map(|v| {
                    let label = v.label.clone();
                    (label, v)
                })
                .collect(),
        }
    }
}
