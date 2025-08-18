use vello::wgpu::PresentMode;

use crate::*;

pub struct SurfaceBuilder {
    pub(crate) width: u32,
    pub(crate) height: u32,

    pub(crate) x: u32,
    pub(crate) y: u32,

    pub(crate) layer: Layer,

    pub(crate) anchor: Anchor,

    pub(crate) title: String,

    pub(crate) window: bool,

    pub(crate) keyboard: KeyboardInteractivity,
}

impl Default for SurfaceBuilder {
    fn default() -> Self {
        SurfaceBuilder {
            width: 800,
            height: 600,

            x: 0,
            y: 0,

            layer: Layer::Top,

            anchor: Anchor::None,

            title: "WLWGPU".to_string(),

            window: false,

            keyboard: KeyboardInteractivity::None,
        }
    }
}

impl SurfaceBuilder {
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn x(mut self, x: u32) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: u32) -> Self {
        self.y = y;
        self
    }

    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    pub fn window(mut self, window: bool) -> Self {
        self.window = window;
        self
    }

    pub fn keyboard(mut self, keyboard: KeyboardInteractivity) -> Self {
        self.keyboard = keyboard;
        self
    }

    pub async fn build(self, wlwgpu: &mut WlWgpu) -> Result<SurfaceId> {
        let wl = &wlwgpu.shell.wl;

        let wayland_surface = match self.window {
            true => WlSurfaceHandle::window(wl, self.title, self.width, self.height),
            false => WlSurfaceHandle::layer(
                wl,
                self.x,
                self.y,
                self.width,
                self.height,
                self.layer,
                self.anchor,
                self.keyboard,
            ),
        };

        let wgpu = &mut wlwgpu.shell.wgpu;

        let wgpu_surface = unsafe {
            wgpu.ctx
                .create_render_surface(
                    wgpu.ctx
                        .instance
                        .create_surface_unsafe(wayland_surface.surface_target(wl))?,
                    self.width,
                    self.height,
                    PresentMode::AutoVsync,
                )
                .await
                .map_err(Report::msg)?
        };

        let surface = Surface::new(wgpu_surface, wayland_surface);

        wgpu.register_surface(surface)
    }
}
