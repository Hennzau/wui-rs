use winit::{
    dpi::PhysicalPosition,
    event::{ButtonSource, MouseScrollDelta},
    keyboard::{Key, ModifiersState},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    KeyPressed(Key),

    KeyReleased(Key),

    KeyModifiersChanged(ModifiersState),

    PointerEntered,
    PointerLeft,

    PointerMoved(PhysicalPosition<f64>),
    PointerPressed {
        position: PhysicalPosition<f64>,
        button: ButtonSource,
    },
    PointerReleased {
        position: PhysicalPosition<f64>,
        button: ButtonSource,
    },

    PointerScrolled(MouseScrollDelta),
}
