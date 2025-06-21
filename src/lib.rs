mod application;
mod element;

mod message;
mod update;
mod view;

pub mod prelude {
    pub use eyre::{Report, Result};

    pub use smithay_client_toolkit::shell::wlr_layer::Anchor;
    pub use wgpu::{Color, hal::Rect};

    pub use crate::application::*;
    pub use crate::element::*;
    pub use crate::message::*;
    pub use crate::update::*;
    pub use crate::view::*;
}
