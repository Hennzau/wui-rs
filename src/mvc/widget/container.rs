use crate::prelude::*;

pub struct ContainerWidget<Message> {
    pub(crate) elements: Vec<Element<Message>>,
}

impl<Message> ContainerWidget<Message> {
    pub fn with(mut self, element: impl IntoElement<Message>) -> Self {
        self.elements.push(element.element());

        self
    }

    pub fn elements(self) -> Vec<Element<Message>> {
        self.elements
    }
}

impl<Message: 'static> Widget<Message> for ContainerWidget<Message> {}

pub fn container<Message>() -> ContainerWidget<Message> {
    ContainerWidget {
        elements: Vec::new(),
    }
}
