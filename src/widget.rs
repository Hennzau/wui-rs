use ::std::any::Any;
use eyre::OptionExt;

use crate::*;

mod root;
pub use root::*;

mod surface;
pub use surface::*;

mod std;
pub use std::*;

pub mod util;

pub trait AnyWidget {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait Widget<Message>: Send + Sync + Any {
    fn size(&self) -> Size;

    fn insets(&self) -> Insets {
        Insets::ZERO
    }

    fn active(&self) -> bool {
        false
    }

    fn merge(self: Box<Self>, element: Element<Message>) -> Element<Message>;

    #[allow(unused_variables)]
    fn handle_event(&mut self, msg: &mut Vec<Message>, event: Event) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn draw(&self, scene: &mut Scene, transform: Affine) -> Result<()> {
        Ok(())
    }
}

impl<Message: 'static> AnyWidget for dyn Widget<Message> {
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
    pub fn size(&self) -> Size {
        self.widget.size()
    }

    pub fn insets(&self) -> Insets {
        self.widget.insets()
    }

    pub fn active(&self) -> bool {
        self.widget.active()
    }

    pub fn handle_event(&mut self, msg: &mut Vec<Message>, event: Event) -> Result<()> {
        self.widget.handle_event(msg, event)
    }

    pub fn draw(&self, scene: &mut Scene, transform: Affine) -> Result<()> {
        self.widget.draw(scene, transform)
    }

    pub fn merge(self, element: Element<Message>) -> Element<Message> {
        self.widget.merge(element)
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
    fn into_element(self) -> Element<Message>;
}

impl<Message, T> IntoElement<Message> for T
where
    T: Widget<Message> + 'static,
{
    fn into_element(self) -> Element<Message> {
        Element {
            widget: Box::new(self),
        }
    }
}
