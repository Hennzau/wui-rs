use crate::*;

pub struct Circle {
    pub(crate) radius: f64,
    pub(crate) color: Color,
}

impl<Message: 'static> Widget<Message> for Circle {
    fn size(&self) -> Size {
        let diameter = self.radius * 2.0;

        Size::new(diameter, diameter)
    }

    fn handle_event(&mut self, _: &mut Vec<Message>, _: Event) -> Result<()> {
        Ok(())
    }

    fn draw(&self, scene: &mut Scene, transform: Affine) -> Result<()> {
        let circle = vello::kurbo::Circle::new((self.radius, self.radius), self.radius);

        scene.fill(Fill::NonZero, transform, self.color, None, &circle);

        Ok(())
    }

    fn merge(self: Box<Self>, element: Element<Message>) -> Element<Message> {
        element
    }
}

impl Circle {
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

pub fn circle() -> Circle {
    Circle {
        radius: 50.0,
        color: palette::css::RED,
    }
}
