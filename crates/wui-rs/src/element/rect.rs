use tokio::sync::mpsc::UnboundedSender;

use crate::prelude::*;

pub struct Rect<Message: 'static + Send + Sync> {
    child: Option<Element<Message>>,
}

impl<Message: 'static + Send + Sync> Widget<Message> for Rect<Message> {
    fn on_event(&mut self, messages: &UnboundedSender<Message>, event: Event) -> Result<()> {
        if let Some(child) = &mut self.child {
            child.on_event(messages, event)?;
        }

        Ok(())
    }

    fn render(&self) {
        self.child.as_ref().map(|child| child.render());
    }
}

impl<Message: 'static + Send + Sync> Rect<Message> {
    fn new(child: Option<impl Into<Element<Message>>>) -> Self {
        Self {
            child: child.map(Into::into),
        }
    }

    pub fn child(self, child: impl Into<Element<Message>>) -> Self {
        Self {
            child: Some(child.into()),
        }
    }

    pub fn size(self, _width: u32, _height: u32) -> Self {
        // This is a placeholder for setting size; actual implementation may vary.
        // In a real application, you would store the size and use it in rendering.
        self
    }
}

impl<Message: 'static + Send + Sync> From<Rect<Message>> for Element<Message> {
    fn from(value: Rect<Message>) -> Self {
        Element::new(Box::new(value))
    }
}

pub fn rect<Message: 'static + Send + Sync>() -> Rect<Message> {
    Rect::new(None::<Element<Message>>)
}
