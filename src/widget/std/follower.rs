use vello::{
    Scene,
    kurbo::{Affine, Circle},
    peniko::Color,
};

use crate::*;

pub struct Follower {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl<Message> Widget<Message> for Follower {
    fn handle_event(&mut self, _: &mut Vec<Message>, event: Event) -> Result<()> {
        if let Event::PointerMoved(position) = event {
            self.x = position.x;
            self.y = position.y;
        }

        Ok(())
    }

    fn draw(&self, scene: &mut Scene) -> Result<()> {
        let circle = Circle::new((0.0, 0.0), 20.0);
        let circle_fill_color = Color::new([0.9529, 0.5451, 0.6588, 1.]);

        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::translate((self.x, self.y)),
            circle_fill_color,
            None,
            &circle,
        );

        Ok(())
    }
}

pub fn follower<Message>() -> Follower {
    Follower { x: 0.0, y: 0.0 }
}
