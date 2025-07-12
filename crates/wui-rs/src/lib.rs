pub mod application;
pub mod element;

pub mod prelude {
    pub use eyre::{Report, Result};

    pub use crate::application::*;
    pub use crate::element::*;
}
