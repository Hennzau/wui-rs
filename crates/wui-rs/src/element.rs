use tokio::sync::mpsc::UnboundedSender;

use crate::prelude::*;

pub mod rect;
pub use rect::*;

pub mod view;
pub use view::*;

pub trait Widget<Message: 'static + Send + Sync>: Send + Sync {
    fn on_event(&mut self, messages: &UnboundedSender<Message>, event: Event) -> Result<()>;

    fn render(&self);
}

pub struct Element<Message: 'static + Send + Sync> {
    pub(crate) widget: Box<dyn Widget<Message>>,
}

impl<Message: 'static + Send + Sync> Element<Message> {
    pub fn new(widget: Box<dyn Widget<Message>>) -> Self {
        Self { widget }
    }

    pub fn on_event(&mut self, messages: &UnboundedSender<Message>, event: Event) -> Result<()> {
        self.widget.on_event(messages, event)
    }

    pub fn render(&self) {
        self.widget.render();
    }
}

impl<Message: 'static + Send + Sync> From<Box<dyn Widget<Message>>> for Element<Message> {
    fn from(widget: Box<dyn Widget<Message>>) -> Self {
        Self::new(widget)
    }
}
