use crate::prelude::*;

pub struct Text<M: Send + Sync + 'static> {
    _marker: std::marker::PhantomData<M>,
}

impl<M: Send + Sync + 'static> Default for Text<M> {
    fn default() -> Self {
        Text {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<M: Send + Sync + 'static> Text<M> {
    pub fn with_string(self, _: impl Into<String>) -> Self {
        self
    }

    pub fn with_color(self, _: Color) -> Self {
        self
    }
}

pub fn text<M: Send + Sync + 'static>() -> Text<M> {
    Text::default()
}

impl<M: Send + Sync + 'static> Into<Element<M>> for Text<M> {
    fn into(self) -> Element<M> {
        Element::none()
    }
}
