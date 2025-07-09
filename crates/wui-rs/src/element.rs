use tokio::task::JoinHandle;

use crate::prelude::*;

pub mod rect;
pub use rect::*;

pub trait Element: Send + Sync {}

pub trait ElementBuilder: Send + Sync {
    fn build(self: Box<Self>, client: Client) -> JoinHandle<Result<ElementHandle>>;
}

pub enum ElementHandle {
    SelfContained(Box<dyn Element>),
    External,
}

impl ElementHandle {}
