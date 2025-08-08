use std::any::Any;

use crate::prelude::*;

mod empty;
pub use empty::*;

mod map;
pub use map::*;

mod container;
pub use container::*;

pub trait AnyWidget: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub type Label = String;
pub type Scene = vello::Scene;

#[derive(Default, Clone)]
pub enum KeyboardInteraction {
    #[default]
    None,
    Aware,
    Exclusive,
}

#[derive(Default, Clone)]
pub enum MouseInteraction {
    #[default]
    None,
    Aware,
}

#[derive(Default, Clone)]
pub enum LayerKind {
    #[default]
    Top,
    Background,
}

#[derive(Default, Clone)]
pub enum Side {
    #[default]
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Default, Clone)]
pub struct Location {
    pub x: u32,
    pub y: u32,

    pub side: Option<Side>,
    pub exclusive: u32,
}

#[derive(Clone)]
pub enum DisplayMode {
    Layered { location: Location, kind: LayerKind },
    Windowed,
}

impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::Layered {
            location: Location::default(),
            kind: LayerKind::default(),
        }
    }
}

#[derive(Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Default for Size {
    fn default() -> Self {
        Size {
            width: 256,
            height: 128,
        }
    }
}

pub trait Widget<Message>: Send + Sync + Any {
    #[allow(unused_variables)]
    fn on_event(&mut self, event: Event, msg: Sender<Message>) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn draw(&self, scene: &mut Scene) -> Result<()> {
        Ok(())
    }

    fn label(&self) -> Option<Label> {
        None
    }

    fn mouse(&self) -> MouseInteraction {
        MouseInteraction::default()
    }

    fn keyboard(&self) -> KeyboardInteraction {
        KeyboardInteraction::default()
    }

    fn display_mode(&self) -> DisplayMode {
        DisplayMode::default()
    }

    fn size(&self) -> Size {
        Size::default()
    }
}

impl<Message: 'static + Send + Sync> AnyWidget for dyn Widget<Message> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

pub struct Element<Message> {
    pub(crate) widget: Box<dyn Widget<Message>>,
}

impl<Message: 'static> Element<Message> {
    pub fn on_event(&mut self, event: Event, msg: Sender<Message>) -> Result<()> {
        self.widget.on_event(event, msg)
    }

    pub fn draw(&self, scene: &mut Scene) -> Result<()> {
        self.widget.draw(scene)
    }

    pub fn label(&self) -> Option<Label> {
        self.widget.label()
    }

    pub fn mouse(&self) -> MouseInteraction {
        self.widget.mouse()
    }

    pub fn keyboard(&self) -> KeyboardInteraction {
        self.widget.keyboard()
    }

    pub fn display_mode(&self) -> DisplayMode {
        self.widget.display_mode()
    }

    pub fn size(&self) -> Size {
        self.widget.size()
    }

    pub fn map<NewMessage: 'static + Send + Sync>(
        self,
        map: Map<Message, NewMessage>,
    ) -> Element<NewMessage> {
        Element {
            widget: Box::new(MapWidget::new(self, map)),
        }
    }
}

impl<Message: 'static + Send + Sync> Element<Message> {
    pub fn into_list(self) -> Vec<Element<Message>> {
        match self.downcast::<ContainerWidget<Message>>() {
            Ok(container) => container.elements,
            Err(element) => vec![element],
        }
    }

    pub fn labels(&self) -> Vec<Option<String>> {
        match self.downcast_ref::<ContainerWidget<Message>>() {
            Ok(container) => container.elements.iter().map(|e| e.label()).collect(),
            Err(_) => vec![self.label()],
        }
    }

    pub fn downcast_ref<T: Widget<Message>>(&self) -> Result<&T> {
        self.widget
            .as_any()
            .downcast_ref::<T>()
            .ok_or_eyre("Failed to downcast Element")
    }

    pub fn downcast_mut<T: Widget<Message>>(&mut self) -> Result<&mut T> {
        self.widget
            .as_any_mut()
            .downcast_mut::<T>()
            .ok_or_eyre("Failed to downcast Element")
    }

    pub fn downcast<T: Widget<Message>>(self) -> Result<Box<T>, Self> {
        if self.widget.as_any().downcast_ref::<T>().is_some() {
            return Ok(self
                .widget
                .into_any()
                .downcast::<T>()
                .expect("Downcasting should have worked..."));
        }

        Err(self)
    }
}

pub trait IntoElement<Message> {
    fn element(self) -> Element<Message>;
}

impl<Message, T> IntoElement<Message> for T
where
    T: Widget<Message> + 'static,
{
    fn element(self) -> Element<Message> {
        Element {
            widget: Box::new(self),
        }
    }
}
