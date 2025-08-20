use crate::*;

pub trait Widget<Message>: Send + Sync {
    #[allow(unused_variables)]
    fn handle_event(&mut self, msg: &mut Vec<Message>, event: EventKind) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn draw(&self, scene: &mut Scene) -> Result<()> {
        Ok(())
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
