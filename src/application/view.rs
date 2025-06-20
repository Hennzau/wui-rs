use crate::prelude::*;

pub struct View<T, M: Send + Sync + 'static> {
    pub element: Box<dyn Fn(&T) -> Element<M>>,
}

impl<T, M: Send + Sync + 'static> View<T, M> {
    pub fn new(element: impl Fn(&T) -> Element<M> + 'static) -> Self {
        View {
            element: Box::new(element),
        }
    }

    pub fn render(&self) {}
}
