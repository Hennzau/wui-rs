use crate::*;

pub struct Follower {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl<Message> Widget<Message> for Follower {
    fn handle_event(&mut self, _: &mut Vec<Message>, event: EventKind) -> Result<()> {
        match event {
            EventKind::PointerMoved { x, y } => {
                self.x = x;
                self.y = y;
            }
            _ => {}
        }

        Ok(())
    }

    fn draw(&self, scene: &mut Scene) -> Result<()> {
        scene.add_circle(self.x, self.y, 20.0, palette::css::FIREBRICK);

        Ok(())
    }
}

pub fn follower<Message>() -> Follower {
    Follower { x: 0.0, y: 0.0 }
}
