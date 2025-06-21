mod button;
mod container;
mod square;
mod text;

pub use button::*;
pub use container::*;
pub use square::*;
pub use text::*;

pub struct Element<M: Send + Sync + 'static> {
    _marker: std::marker::PhantomData<M>,
}

impl<M: Send + Sync + 'static> Element<M> {
    pub fn none() -> Self {
        Element {
            _marker: std::marker::PhantomData,
        }
    }
}
