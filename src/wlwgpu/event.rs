use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum EventKind {
    Close,
    Resize {
        width: u32,
        height: u32,
    },

    Draw,

    KeyboardEntered,
    KeyboardLeaved,

    KeyPressed {
        key: u32,
    },

    KeyReleased {
        key: u32,
    },

    KeyModifiersChanged {
        ctrl: bool,
        alt: bool,
        shift: bool,
        caps_lock: bool,
        logo: bool,
        num_lock: bool,
    },

    PointerEntered,
    PointerLeaved,

    PointerMoved {
        x: f64,
        y: f64,
    },
    PointerPressed {
        x: f64,
        y: f64,
        button: u32,
    },
    PointerReleased {
        x: f64,
        y: f64,
        button: u32,
    },

    PointerScrolled {
        x: f64,
        y: f64,

        delta_x: f64,
        delta_y: f64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub id: Option<SurfaceId>,
    pub kind: EventKind,
}
