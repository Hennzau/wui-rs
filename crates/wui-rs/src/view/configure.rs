use crate::prelude::*;

impl View {
    pub fn configure(&self, configure: WaylandViewConfigure) {
        let (width, height) = match configure {
            WaylandViewConfigure::Window(configure) => (
                configure.new_size.0.unwrap().get(),
                configure.new_size.1.unwrap().get(),
            ),
            WaylandViewConfigure::LayerSurface(configure) => {
                (configure.new_size.0, configure.new_size.1)
            }
        };

        let cap = self.surface.get_capabilities(&self.adapter);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: cap.formats[0],
            view_formats: vec![cap.formats[0]],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: width,
            height: height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        self.surface.configure(&self.device, &surface_config);

        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let _renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
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
