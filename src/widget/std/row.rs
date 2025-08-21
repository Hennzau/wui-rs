use crate::*;

pub struct Row<Message> {
    pub(crate) children: Vec<Element<Message>>,
}

impl<Message: 'static> Widget<Message> for Row<Message> {
    fn size(&self) -> Size {
        self.children.iter().fold(Size::ZERO, |acc, child| {
            let child_size = child.size();

            Size::new(
                acc.width + child_size.width,
                acc.height.max(child_size.height),
            )
        })
    }

    fn handle_event(&mut self, msg: &mut Vec<Message>, event: Event) -> Result<()> {
        let mut transform = Affine::IDENTITY;

        for child in &mut self.children {
            util::transform_handle_event(child, msg, event.clone(), transform)?;

            transform = transform.pre_translate((-child.size().width, 0.0).into());
        }

        Ok(())
    }

    fn draw(&self, scene: &mut Scene, mut transform: Affine) -> Result<()> {
        for child in &self.children {
            child.draw(scene, transform)?;

            transform = transform * Affine::translate((child.size().width, 0.0));
        }

        Ok(())
    }

    fn merge(self: Box<Self>, element: Element<Message>) -> Element<Message> {
        match element.downcast::<Self>() {
            Ok(mut element) => {
                if self.children.len() != element.children.len() {
                    return element.into_element(); // If the number of children differs, do not merge
                }

                let other_children = element.children.drain(..).collect::<Vec<_>>();

                for (child, other_child) in self.children.into_iter().zip(other_children) {
                    element.children.push(child.merge(other_child));
                }

                element.into_element()
            }
            Err(element) => element,
        }
    }
}

impl<Message> Row<Message> {
    pub fn child(mut self, child: impl IntoElement<Message>) -> Self {
        self.children.push(child.into_element());
        self
    }

    pub fn child_if(mut self, v: bool, child: impl IntoElement<Message>) -> Self {
        if v {
            self.children.push(child.into_element());
        }

        self
    }
}

pub fn row<Message>() -> Row<Message> {
    Row {
        children: Vec::new(),
    }
}
