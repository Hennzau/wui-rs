use std::ptr::NonNull;

use raw_window_handle::{
    RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
};
use vello::Scene;
use wayland_backend::client::ObjectId;
use wayland_client::Connection;

use crate::prelude::*;

pub struct Renderer {
    pub(crate) vello: Vec<Option<vello::Renderer>>,
    pub(crate) context: vello::util::RenderContext,
}

impl Renderer {
    pub(crate) fn new() -> Result<Self> {
        let context = vello::util::RenderContext::new();

        let vello = Vec::new();

        Ok(Self { context, vello })
    }

    pub(crate) async fn create_surface(
        &mut self,
        width: u32,
        height: u32,
        connection: &Connection,
        id: ObjectId,
    ) -> Result<vello::util::RenderSurface<'static>> {
        let raw_display_handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
            NonNull::new(connection.backend().display_ptr() as *mut _).unwrap(),
        ));

        let raw_window_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(
            NonNull::new(id.as_ptr() as *mut _).unwrap(),
        ));

        let surface = unsafe {
            self.context
                .create_render_surface(
                    self.context.instance.create_surface_unsafe(
                        wgpu::SurfaceTargetUnsafe::RawHandle {
                            raw_display_handle,
                            raw_window_handle,
                        },
                    )?,
                    width,
                    height,
                    wgpu::PresentMode::AutoVsync,
                )
                .await
                .map_err(Report::msg)?
        };

        self.vello.resize_with(self.context.devices.len(), || None);
        self.vello[surface.dev_id].get_or_insert(vello::Renderer::new(
            &self.context.devices[surface.dev_id].device,
            vello::RendererOptions::default(),
        )?);

        Ok(surface)
    }

    pub(crate) fn render(&mut self, surface: &Surface, scene: &Scene) -> Result<()> {
        let device = &self.context.devices[surface.wgpu_surface.dev_id];

        self.vello[surface.wgpu_surface.dev_id]
            .as_mut()
            .ok_or_eyre("No renderer for this surface")?
            .render_to_texture(
                &device.device,
                &device.queue,
                scene,
                &surface.wgpu_surface.target_view,
                &vello::RenderParams {
                    base_color: vello::peniko::color::palette::css::BLACK,
                    width: surface.wgpu_surface.config.width,
                    height: surface.wgpu_surface.config.height,
                    antialiasing_method: vello::AaConfig::Msaa16,
                },
            )?;

        let surface_texture = surface
            .wgpu_surface
            .surface
            .get_current_texture()
            .expect("failed to get surface texture");

        let mut encoder = device
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Surface Blit"),
            });

        surface.wgpu_surface.blitter.copy(
            &device.device,
            &mut encoder,
            &surface.wgpu_surface.target_view,
            &surface_texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
        );

        device.queue.submit([encoder.finish()]);
        surface_texture.present();

        Ok(())
    }

    pub(crate) fn configure(
        &mut self,
        surface: &mut Surface,
        width: u32,
        height: u32,
    ) -> Result<()> {
        self.context
            .resize_surface(&mut surface.wgpu_surface, width, height);

        let mut scene = Scene::new();
        let rect = vello::kurbo::Rect::new(0.0, 0.0, width as f64, height as f64);
        let rect_fill_color = vello::peniko::Color::BLACK;
        scene.fill(
            vello::peniko::Fill::NonZero,
            vello::kurbo::Affine::IDENTITY,
            rect_fill_color,
            None,
            &rect,
        );

        self.render(surface, &scene)
    }
}
