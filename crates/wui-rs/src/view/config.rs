use smithay_client_toolkit::shell::{
    wlr_layer::LayerSurfaceConfigure, xdg::window::WindowConfigure,
};

pub use smithay_client_toolkit::shell::wlr_layer::{Anchor, KeyboardInteractivity, Layer};
pub use smithay_client_toolkit::shell::xdg::window::WindowDecorations;

#[derive(Debug, Clone)]
pub(crate) enum ViewConfigure {
    LayerSurface(LayerSurfaceConfigure),
    Window(WindowConfigure),
}

#[derive(Debug, Clone)]
pub struct ViewConfiguration {
    pub namespace: Option<String>,
    pub layer: Layer,
    pub anchor: Anchor,
    pub keyboard_interactivity: KeyboardInteractivity,
    pub size: (u32, u32),
    pub min_size: Option<(u32, u32)>,
    pub max_size: Option<(u32, u32)>,
    pub margin: (i32, i32, i32, i32),
    pub exclusive_zone: i32,
    pub decorations: WindowDecorations,
    pub title: String,
    pub app_id: String,
}

impl Default for ViewConfiguration {
    fn default() -> Self {
        Self {
            namespace: None,
            layer: Layer::Top,
            anchor: Anchor::TOP,
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            size: (0, 0),
            min_size: None,
            max_size: None,
            margin: (0, 0, 0, 0),
            exclusive_zone: 0,
            decorations: WindowDecorations::ServerDefault,
            title: String::from("wui_rs.default"),
            app_id: String::from("io.github.wui_rs.default"),
        }
    }
}
