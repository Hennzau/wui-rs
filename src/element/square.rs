use crate::prelude::*;

pub struct Square<M: Send + Sync + 'static> {
    _marker: std::marker::PhantomData<M>,
}

impl<M: Send + Sync + 'static> Default for Square<M> {
    fn default() -> Self {
        Square {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<M: Send + Sync + 'static> Square<M> {
    pub fn enabled(self, _: bool) -> Self {
        self
    }

    pub fn with_rect(self, _: Rect<u32>) -> Self {
        self
    }

    pub fn with_color(self, _: Color) -> Self {
        self
    }

    pub fn with_child(self, _: Element<M>) -> Self {
        self
    }
}

pub fn square<M: Send + Sync + 'static>() -> Square<M> {
    Square::default()
}

impl<M: Send + Sync + 'static> Into<Element<M>> for Square<M> {
    fn into(self) -> Element<M> {
        Element::none()
    }
}
