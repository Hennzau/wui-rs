pub use winit::{
    event::ButtonSource,
    keyboard::{Key, ModifiersState},
};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseScrollDelta {
    LineDelta(f32, f32),
    PixelDelta(Vec2),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    KeyPressed(Key),

    KeyReleased(Key),

    KeyModifiersChanged(ModifiersState),

    PointerEntered,
    PointerLeft,

    PointerMoved(Point),
    PointerPressed {
        position: Point,
        button: ButtonSource,
    },
    PointerReleased {
        position: Point,
        button: ButtonSource,
    },

    PointerScrolled(MouseScrollDelta),
}
