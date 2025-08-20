use vello::peniko::Color;

use winit::{
    dpi::{LogicalPosition, LogicalSize},
    platform::wayland::WindowAttributesWayland,
    window::WindowAttributes,
};

pub use winit::platform::wayland::{Anchor, KeyboardInteractivity, Layer};

use crate::*;

pub struct RootWidget<Message> {
    pub(crate) label: String,
    pub(crate) windowed: bool,

    pub(crate) layer: Option<Layer>,
    pub(crate) anchor: Option<Anchor>,
    pub(crate) exclusive_zone: Option<i32>,
    pub(crate) keyboard_interactivity: Option<KeyboardInteractivity>,

    pub(crate) size: Option<LogicalSize<i32>>,
    pub(crate) position: Option<LogicalPosition<i32>>,

    pub(crate) background: Color,

    pub(crate) child: Option<Element<Message>>,
}

pub fn root<Message>(label: impl Into<String>) -> RootWidget<Message> {
    RootWidget {
        label: label.into(),
        windowed: false,

        layer: None,
        anchor: None,
        exclusive_zone: None,
        keyboard_interactivity: None,

        size: None,
        position: None,

        background: Color::BLACK,

        child: None,
    }
}

impl<Message> RootWidget<Message> {
    pub fn windowed(mut self) -> Self {
        self.windowed = true;
        self
    }

    pub fn layer(mut self, layer: Layer) -> Self {
        self.layer = Some(layer);
        self
    }

    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = Some(anchor);
        self
    }

    pub fn exclusive_zone(mut self, exclusive_zone: i32) -> Self {
        self.exclusive_zone = Some(exclusive_zone);
        self
    }

    pub fn keyboard_interactivity(mut self, keyboard_interactivity: KeyboardInteractivity) -> Self {
        self.keyboard_interactivity = Some(keyboard_interactivity);
        self
    }

    pub fn size(mut self, size: LogicalSize<i32>) -> Self {
        self.size = Some(size);
        self
    }

    pub fn position(mut self, position: LogicalPosition<i32>) -> Self {
        self.position = Some(position);
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    pub fn child(mut self, child: impl IntoElement<Message>) -> Self {
        self.child = Some(child.element());
        self
    }

    pub fn as_attributes(&self) -> WindowAttributes {
        let mut attributes = WindowAttributes::default()
            .with_title(self.label.clone())
            .with_transparent(true);

        if let Some(size) = self.size {
            attributes = attributes.with_surface_size(size);
        }

        if let Some(position) = self.position
            && self.windowed
        {
            attributes = attributes.with_position(position);
        }

        let mut wayland_attributes = WindowAttributesWayland::default();

        if !self.windowed {
            wayland_attributes = wayland_attributes.with_layer_shell();

            if let Some(position) = self.position {
                wayland_attributes = wayland_attributes.with_margin(position.y, 0, 0, position.x);
            }
        }

        if let Some(layer) = self.layer {
            wayland_attributes = wayland_attributes.with_layer(layer);
        }

        if let Some(anchor) = self.anchor {
            wayland_attributes = wayland_attributes.with_anchor(anchor);
        }

        if let Some(exclusive_zone) = self.exclusive_zone {
            wayland_attributes = wayland_attributes.with_exclusive_zone(exclusive_zone);
        }

        if let Some(keyboard_interactivity) = self.keyboard_interactivity {
            wayland_attributes =
                wayland_attributes.with_keyboard_interactivity(keyboard_interactivity);
        }

        attributes = attributes.with_platform_attributes(Box::new(wayland_attributes));

        attributes
    }
}

impl<Message: 'static> Widget<Message> for RootWidget<Message> {}

pub trait IntoRootWidgets<Message> {
    fn into_root_widgets(self) -> Vec<RootWidget<Message>>;
}

impl<Message> IntoRootWidgets<Message> for RootWidget<Message> {
    fn into_root_widgets(self) -> Vec<RootWidget<Message>> {
        vec![self]
    }
}
impl<Message> IntoRootWidgets<Message> for Vec<RootWidget<Message>> {
    fn into_root_widgets(self) -> Vec<RootWidget<Message>> {
        self
    }
}
