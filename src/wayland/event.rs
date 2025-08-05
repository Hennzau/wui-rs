use crate::prelude::*;

#[derive(Debug, Clone)]
pub(crate) enum WaylandWidgetEvent {
    Close,

    /// Configuration event that provides the width and height for the widget in case
    /// the widget needs to be resized or initialized with specific dimensions.
    Configure {
        width: u32,
        height: u32,
    },

    /// Render event that indicates the widget should be redrawn. This event should only be used
    /// on the master Widget provided to the `Backend`.
    Draw,

    WidgetEvent(Event),
}
