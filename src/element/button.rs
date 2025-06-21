use crate::prelude::*;

pub enum ButtonEvent {}

pub struct Button<M: Send + Sync + 'static> {
    _marker: std::marker::PhantomData<M>,
}

impl<M: Send + Sync + 'static> Default for Button<M> {
    fn default() -> Self {
        Button {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<M: Send + Sync + 'static> Button<M> {
    pub fn enabled(self, _: bool) -> Self {
        self
    }

    pub fn with_rect(self, _: Rect<u32>) -> Self {
        self
    }

    pub fn with_color(self, _: Color) -> Self {
        self
    }

    pub fn on_click(self, _: impl Fn(ButtonEvent) -> M) -> Self {
        self
    }
}

pub fn button<M: Send + Sync + 'static>() -> Button<M> {
    Button::default()
}

impl<M: Send + Sync + 'static> Into<Element<M>> for Button<M> {
    fn into(self) -> Element<M> {
        Element::none()
    }
}
