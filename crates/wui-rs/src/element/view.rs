use smithay_client_toolkit::{
    seat::keyboard::{Keysym, Modifiers},
    shell::wlr_layer::{Anchor, KeyboardInteractivity},
};
use tokio::{sync::mpsc::UnboundedSender, task::JoinHandle};
use wayland_backend::server::ObjectId;

use crate::prelude::*;

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
    id: Option<ObjectId>,

    label: Option<String>,
    anchor: Option<Anchor>,
    keyboard_interactivity: Option<KeyboardInteractivity>,
    size: Option<(u32, u32)>,
    exclusive_zone: Option<i32>,
    position: Option<(i32, i32)>,

    child: Option<Element<Message>>,
}

impl<Message: 'static + Send + Sync> Widget<Message> for View<Message> {
    fn build(
        mut self: Box<Self>,
        client: Client<Message>,
    ) -> JoinHandle<Option<Box<dyn Widget<Message>>>> {
        tokio::spawn(async move {
            if let Some(child) = self.child {
                if let Ok(Some(child)) = child.build(client).await {
                    self.child = Some(child);
                }
            }

            None
        })
    }

    fn on_event(&mut self, messages: &UnboundedSender<Message>, event: Event) {
        self.child.as_mut().map(|child| {
            child.on_event(messages, event);
        });
    }

    fn render(&self) {
        self.child.as_ref().map(|child| {
            child.render();
        });
    }
}

impl<Message: 'static + Send + Sync> View<Message> {
    fn new() -> Self {
        Self {
            id: None,
            child: None,
            label: None,
            anchor: None,
            keyboard_interactivity: None,
            size: None,
            exclusive_zone: None,
            position: None,
        }
    }

    pub fn child(self, child: impl Into<Element<Message>>) -> Self {
        Self {
            child: Some(child.into()),
            ..self
        }
    }

    pub fn label(self, label: impl Into<String>) -> Self {
        Self {
            label: Some(label.into()),
            ..self
        }
    }

    pub fn anchor(self, anchor: Anchor) -> Self {
        Self {
            anchor: Some(anchor),
            ..self
        }
    }

    pub fn keyboard_interactivity(self, keyboard_interactivity: KeyboardInteractivity) -> Self {
        Self {
            keyboard_interactivity: Some(keyboard_interactivity),
            ..self
        }
    }

    pub fn size(self, width: u32, height: u32) -> Self {
        Self {
            size: Some((width, height)),
            ..self
        }
    }

    pub fn exclusive_zone(self, exclusive_zone: i32) -> Self {
        Self {
            exclusive_zone: Some(exclusive_zone),
            ..self
        }
    }

    pub fn position(self, x: i32, y: i32) -> Self {
        Self {
            position: Some((x, y)),
            ..self
        }
    }
}

pub fn view<Message: 'static + Send + Sync>() -> View<Message> {
    View::new()
}

pub struct Views<Message: 'static + Send + Sync>(pub Vec<View<Message>>);

impl<Message: 'static + Send + Sync> From<View<Message>> for Views<Message> {
    fn from(value: View<Message>) -> Self {
        Views(vec![value])
    }
}
impl<Message: 'static + Send + Sync> From<Vec<View<Message>>> for Views<Message> {
    fn from(value: Vec<View<Message>>) -> Self {
        Self(value)
    }
}
