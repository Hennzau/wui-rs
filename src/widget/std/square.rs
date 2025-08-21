use crate::*;

pub struct Square {
    pub(crate) size: Size,
    pub(crate) color: Color,
}

impl<Message: 'static> Widget<Message> for Square {
    fn size(&self) -> Size {
        self.size
    }

    fn handle_event(&mut self, _: &mut Vec<Message>, _: Event) -> Result<()> {
        Ok(())
    }

    fn draw(&self, scene: &mut Scene, transform: Affine) -> Result<()> {
        scene.fill(
            Fill::NonZero,
            transform,
            self.color,
            None,
            &self.size.to_rect(),
        );

        Ok(())
    }

    fn merge(self: Box<Self>, element: Element<Message>) -> Element<Message> {
        element
    }
}

impl Square {
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

pub fn square() -> Square {
    Square {
        size: Size::new(100.0, 100.0),
        color: palette::css::RED,
    }
}
