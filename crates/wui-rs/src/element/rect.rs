use tokio::task::JoinHandle;

use crate::prelude::*;

pub struct Rect {}

impl Element for Rect {}

pub struct RectBuilder {}

pub fn rect() -> Box<RectBuilder> {
    Box::new(RectBuilder::default())
}

impl Default for RectBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl ElementBuilder for RectBuilder {
    fn build(self: Box<Self>, _: Client) -> JoinHandle<Result<ElementHandle>> {
        tokio::task::spawn(async move { Ok(ElementHandle::SelfContained(Box::new(Rect {}))) })
    }
}
