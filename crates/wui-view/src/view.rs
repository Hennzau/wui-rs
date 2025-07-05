use smithay_client_toolkit::{
    seat::{
        keyboard::{KeyEvent, Keysym, Modifiers},
        pointer::PointerEvent,
    },
    shell::{wlr_layer::LayerSurfaceConfigure, xdg::window::WindowConfigure},
};
use wayland_client::protocol::wl_output::Transform;

#[derive(Debug, Clone)]
pub enum ViewConfigure {
    LayerSurface(LayerSurfaceConfigure),
    Window(WindowConfigure),
}

#[derive(Debug, Clone)]
pub enum ViewEvent {
    ScaleFactorChanged(i32),
    TransformChanged(Transform),
    Frame(u32),
    KeyboardEnter(Vec<Keysym>),
    KeyboardLeave,
    KeyPressed(KeyEvent),
    KeyReleased(KeyEvent),
    KeyModifiersChanged(Modifiers),
    Configure(ViewConfigure),
    Closed,
    PointerFrame(PointerEvent),
}
