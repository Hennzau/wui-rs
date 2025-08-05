//! This module only defines the `Event` enum, which represents various events that can occur for
//! a widget.

/// The `Event` enum represents different types of events that can occur for a widget.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// The keyboard entered the widget, meaning it is now focused and can receive keyboard input.
    KeyboardEntered,

    /// The keyboard left the widget, meaning it is no longer focused and will not receive keyboard input.
    KeyboardLeaved,

    /// Key events for keyboard input.
    KeyPressed { key: u32 },

    /// Key events for keyboard input when a key is released.
    KeyReleased { key: u32 },

    /// Key modifiers changed, indicating a change in the state of modifier keys (Ctrl, Alt, Shift, etc.).
    KeyModifiersChanged {
        ctrl: bool,
        alt: bool,
        shift: bool,
        caps_lock: bool,
        logo: bool,
        num_lock: bool,
    },

    /// The pointer entered the widget, meaning it is now focused and can receive pointer input.
    PointerEntered,

    /// The pointer left the widget, meaning it is no longer focused and will not receive pointer input.
    PointerLeaved,

    /// Pointer events for pointer input when the pointer is moved.
    PointerMoved { x: f64, y: f64 },

    /// Pointer events for pointer input when a button is pressed.
    PointerPressed { x: f64, y: f64, button: u32 },

    /// Pointer events for pointer input when a button is released.
    PointerReleased { x: f64, y: f64, button: u32 },

    /// Pointer events for pointer input when the pointer is scrolled.
    PointerScrolled {
        x: f64,
        y: f64,

        delta_x: f64,
        delta_y: f64,
    },
}
