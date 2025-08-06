mod common;
mod mvc;
mod renderer;
mod wayland;

pub mod prelude {
    pub(crate) use eyre::OptionExt;

    pub use eyre::{Report, Result};

    pub use crate::common::*;
    pub use crate::mvc::*;
    pub use crate::renderer::*;
    pub(crate) use crate::wayland::*;
}
