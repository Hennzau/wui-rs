use std::collections::HashMap;

use vello::{
    Renderer,
    util::RenderContext,
    wgpu::{CommandEncoderDescriptor, TextureViewDescriptor},
};

use crate::*;

pub struct Wgpu {
    pub(crate) surfaces: HashMap<SurfaceId, Surface>,
    pub(crate) renderers: Vec<Option<Renderer>>,
    pub(crate) ctx: RenderContext,
}

impl Wgpu {
    pub(crate) fn new() -> Self {
        let ctx = RenderContext::new();
        let surfaces = HashMap::new();
        let renderers = Vec::new();

        Wgpu {
            ctx,
            surfaces,
            renderers,
        }
    }

    pub(crate) fn register_surface(&mut self, surface: Surface) -> Result<SurfaceId> {
        let id = surface.id();

        self.renderers.resize_with(self.ctx.devices.len(), || None);
        self.renderers[surface.dev_id()].get_or_insert(Renderer::new(
            &self.ctx.devices[surface.dev_id()].device,
            vello::RendererOptions::default(),
        )?);

        self.surfaces.insert(id.clone(), surface);

        Ok(id)
    }

    pub fn destroy_surface(&mut self, id: &SurfaceId) {
        if let Some(surface) = self.surfaces.remove(id) {
            surface.destroy();
        }
    }

    pub fn resize_surface(&mut self, id: &SurfaceId, width: u32, height: u32) {
        if let Some(surface) = self.surfaces.get_mut(id) {
            self.ctx.resize_surface(&mut surface.wgpu, width, height);

            // Vello is setting the alpha mode to Auto, which is not suitable here.

            surface.wgpu.config.alpha_mode = vello::wgpu::CompositeAlphaMode::PreMultiplied;
            let device = self
                .ctx
                .devices
                .get(surface.dev_id())
                .expect("Device not found for surface resize");

            surface
                .wgpu
                .surface
                .configure(&device.device, &surface.wgpu.config);
        }
    }

    pub fn size(&self, id: &SurfaceId) -> Result<(u32, u32)> {
        self.surfaces
            .get(id)
            .map(|s| (s.wgpu.config.width, s.wgpu.config.height))
            .ok_or_else(|| eyre::eyre!("Surface not found"))
    }

    pub fn render(&mut self, id: &SurfaceId, scene: &Scene) -> Result<()> {
        let surface = self
            .surfaces
            .get(id)
            .ok_or_else(|| eyre::eyre!("Surface not found"))?;

        let device = self
            .ctx
            .devices
            .get(surface.dev_id())
            .ok_or_else(|| eyre::eyre!("Device not found"))?;

        let renderer = self
            .renderers
            .get_mut(surface.dev_id())
            .and_then(Option::as_mut)
            .ok_or_else(|| eyre::eyre!("No renderer for this surface"))?;

        renderer.render_to_texture(
            &device.device,
            &device.queue,
            scene.as_ref(),
            &surface.wgpu.target_view,
            &vello::RenderParams {
                base_color: vello::peniko::color::palette::css::TRANSPARENT,
                width: surface.wgpu.config.width,
                height: surface.wgpu.config.height,
                antialiasing_method: vello::AaConfig::Msaa16,
            },
        )?;

        let surface_texture = surface.wgpu.surface.get_current_texture()?;

        let mut encoder = device
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Surface Blit"),
            });

        surface.wgpu.blitter.copy(
            &device.device,
            &mut encoder,
            &surface.wgpu.target_view,
            &surface_texture
                .texture
                .create_view(&TextureViewDescriptor::default()),
        );

        device.queue.submit([encoder.finish()]);
        surface_texture.present();

        Ok(())
    }

    pub fn surfaces(&self) -> usize {
        self.surfaces.len()
    }
}
