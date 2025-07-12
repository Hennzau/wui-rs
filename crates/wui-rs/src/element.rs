use crate::prelude::*;

pub mod rect;
pub use rect::*;

pub mod view;
use tokio::{sync::mpsc::UnboundedSender, task::JoinHandle};
pub use view::*;

pub trait Widget<Message: 'static + Send + Sync>: Send + Sync {
    fn build(
        self: Box<Self>,
        client: Client<Message>,
    ) -> JoinHandle<Option<Box<dyn Widget<Message>>>>;

    fn on_event(&mut self, messages: &UnboundedSender<Message>, event: Event);

    fn render(&self);
}

pub struct Element<Message: 'static + Send + Sync> {
    pub(crate) widget: Box<dyn Widget<Message>>,
}

impl<Message: 'static + Send + Sync> Element<Message> {
    pub fn new(widget: Box<dyn Widget<Message>>) -> Self {
        Self { widget }
    }

    pub async fn build(self, client: Client<Message>) -> Result<Option<Self>> {
        let widget = self.widget.build(client).await?;

        Ok(widget.map(|w| Element::new(w)))
    }

    pub fn on_event(&mut self, messages: &UnboundedSender<Message>, event: Event) {
        self.widget.on_event(messages, event);
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
