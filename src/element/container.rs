use crate::prelude::*;

pub struct Container<M: Send + Sync + 'static> {
    _marker: std::marker::PhantomData<M>,
}

impl<M: Send + Sync + 'static> Default for Container<M> {
    fn default() -> Self {
        Container {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<M: Send + Sync + 'static> Container<M> {
    pub fn enabled(self, _: bool) -> Self {
        self
    }

    pub fn with_child(self, _: Element<M>) -> Self {
        self
    }
}

pub fn container<M: Send + Sync + 'static>() -> Container<M> {
    Container::default()
}

impl<M: Send + Sync + 'static> Into<Element<M>> for Container<M> {
    fn into(self) -> Element<M> {
        Element::none()
    }
}
