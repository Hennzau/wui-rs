use smithay_client_toolkit::shell::{
    wlr_layer::LayerSurfaceConfigure, xdg::window::WindowConfigure,
};

use ::wgpu::{
    Color, CompositeAlphaMode, LoadOp, Operations, PresentMode, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
};
pub use smithay_client_toolkit::shell::wlr_layer::{Anchor, KeyboardInteractivity, Layer};
pub use smithay_client_toolkit::shell::xdg::window::WindowDecorations;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub(crate) enum ViewConfigure {
    LayerSurface(LayerSurfaceConfigure),
    Window(WindowConfigure),
}

#[derive(Debug, Clone)]
pub struct ViewConfiguration {
    pub namespace: String,
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
            namespace: String::from("io.app"),
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

impl View {
    pub(crate) fn configure(&self, configure: ViewConfigure) {
        let (width, height) = match configure {
            ViewConfigure::Window(configure) => (
                configure.new_size.0.unwrap().get(),
                configure.new_size.1.unwrap().get(),
            ),
            ViewConfigure::LayerSurface(configure) => (configure.new_size.0, configure.new_size.1),
        };

        let cap = self.surface.get_capabilities(&self.adapter);

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: cap.formats[0],
            view_formats: vec![cap.formats[0]],
            alpha_mode: CompositeAlphaMode::Auto,
            width,
            height,
            desired_maximum_frame_latency: 2,
            present_mode: PresentMode::Mailbox,
        };

        self.surface.configure(&self.device, &surface_config);

        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let _renderpass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLUE),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}
