use crate::*;

pub struct Container<Message> {
    pub(crate) child: Element<Message>,

    pub(crate) margin: Insets,
    pub(crate) color: Color,
}

impl<Message: 'static> Widget<Message> for Container<Message> {
    fn size(&self) -> Size {
        self.child.size() + self.margin.size()
    }

    fn insets(&self) -> Insets {
        -self.margin
    }

    fn handle_event(&mut self, msg: &mut Vec<Message>, event: Event) -> Result<()> {
        util::transform_handle_event(
            &mut self.child,
            msg,
            event,
            Affine::translate((-self.margin.x0, -self.margin.y0)),
        )
    }

    fn draw(&self, scene: &mut Scene, transform: Affine) -> Result<()> {
        scene.fill(
            vello::peniko::Fill::NonZero,
            transform,
            self.color,
            None,
            &self.size().to_rect(),
        );

        self.child.draw(
            scene,
            transform * Affine::translate((self.margin.x0, self.margin.y0)),
        )
    }

    fn merge(self: Box<Self>, element: Element<Message>) -> Element<Message> {
        match element.downcast::<Self>() {
            Ok(mut element) => {
                element.child = self.child.merge(element.child);

                Element { widget: element }
            }
            Err(element) => element,
        }
    }
}

impl<Message> Container<Message> {
    pub fn margin(mut self, margin: Insets) -> Self {
        self.margin = margin;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

pub fn container<Message>(child: impl IntoElement<Message>) -> Container<Message> {
    Container {
        child: child.into_element(),
        margin: Insets::ZERO,
        color: Color::TRANSPARENT,
    }
}
