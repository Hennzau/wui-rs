use std::any::Any;

use crate::*;

pub trait AnyWidget {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait Widget<Message>: Send + Sync + Any {
    #[allow(unused_variables)]
    fn handle_event(&mut self, msg: &mut Vec<Message>, event: EventKind) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn draw(&self, scene: &mut Scene) -> Result<()> {
        Ok(())
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
    pub fn handle_event(&mut self, msg: &mut Vec<Message>, event: EventKind) -> Result<()> {
        self.widget.handle_event(msg, event)
    }

    pub fn draw(&self, scene: &mut Scene) -> Result<()> {
        self.widget.draw(scene)
    }
}
