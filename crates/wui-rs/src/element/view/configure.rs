use eyre::OptionExt;
use wgpu::{
    Color, CompositeAlphaMode, LoadOp, Operations, PresentMode, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
};

use crate::prelude::*;

impl<Message: 'static + Send + Sync> View<Message> {
    pub fn configure(&self, width: u32, height: u32) -> Result<()> {
        let surface = self
            .surface
            .as_ref()
            .ok_or_eyre("Surface is not configured")?;

        let adapter = self
            .adapter
            .as_ref()
            .ok_or_eyre("Adapter is not configured")?;

        let device = self
            .device
            .as_ref()
            .ok_or_eyre("Device is not configured")?;

        let queue = self.queue.as_ref().ok_or_eyre("Queue is not configured")?;

        let cap = surface.get_capabilities(&adapter);

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

        surface.configure(&device, &surface_config);

        let surface_texture = surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");
        let texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&Default::default());
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

        queue.submit(Some(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}
