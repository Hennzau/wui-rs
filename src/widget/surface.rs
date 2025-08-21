use std::sync::Arc;

use vello::{
    AaConfig, Renderer, Scene,
    kurbo::{Affine, Rect},
    peniko::{Color, Fill},
    util::{DeviceHandle, RenderContext, RenderSurface},
    wgpu::{CommandEncoderDescriptor, PresentMode, TextureViewDescriptor},
};
use winit::{
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::*;

pub struct SurfaceWidget<Message> {
    pub(crate) id: WindowId,
    pub(crate) label: String,

    pub(crate) surface: RenderSurface<'static>,
    pub(crate) window: Arc<dyn Window>,

    pub(crate) scene: Scene,

    pub(crate) background: Color,

    pub(crate) child: Option<Element<Message>>,
}

impl<Message: 'static> SurfaceWidget<Message> {
    pub fn new(
        root: RootWidget<Message>,
        event_loop: &dyn ActiveEventLoop,
        ctx: &mut RenderContext,
    ) -> Result<Self> {
        let attributes = root.as_attributes();
        let window = Arc::<dyn Window>::from(event_loop.create_window(attributes)?);

        let size = window.outer_size();
        let surface_future = ctx.create_surface(
            window.clone(),
            size.width,
            size.height,
            PresentMode::AutoVsync,
        );

        let surface = pollster::block_on(surface_future)?;

        Ok(Self {
            id: window.id(),
            window,
            surface,

            background: root.background,

            label: root.label.clone(),
            scene: Scene::new(),
            child: root.child,
        })
    }

    pub fn dev_id(&self) -> usize {
        self.surface.dev_id
    }

    pub(crate) fn handle_event(&mut self, msg: &mut Vec<Message>, event: Event) -> Result<()> {
        if let Some(child) = &mut self.child {
            child.handle_event(msg, event)?;
        }

        Ok(())
    }

    pub(crate) fn draw(&mut self, renderer: &mut Renderer, device: &DeviceHandle) -> Result<()> {
        self.scene.reset();

        let width = self.surface.config.width;
        let height = self.surface.config.height;

        let rect = Rect::new(0.0, 0.0, width as f64, height as f64);

        self.scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            self.background,
            None,
            &rect,
        );

        if let Some(child) = &self.child {
            child.draw(&mut self.scene, Affine::IDENTITY)?;
        }

        renderer.render_to_texture(
            &device.device,
            &device.queue,
            &self.scene,
            &self.surface.target_view,
            &vello::RenderParams {
                base_color: palette::css::TRANSPARENT,
                width,
                height,
                antialiasing_method: AaConfig::Msaa16,
            },
        )?;

        let surface_texture = self
            .surface
            .surface
            .get_current_texture()
            .expect("failed to get surface texture");

        let mut encoder = device
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Surface Blit"),
            });

        self.surface.blitter.copy(
            &device.device,
            &mut encoder,
            &self.surface.target_view,
            &surface_texture
                .texture
                .create_view(&TextureViewDescriptor::default()),
        );

        device.queue.submit([encoder.finish()]);

        self.window.pre_present_notify();

        surface_texture.present();

        Ok(())
    }
}
