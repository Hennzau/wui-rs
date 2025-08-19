use vello::wgpu::PresentMode;

use crate::*;

pub struct WlWgpuWidget<Message> {
    pub(crate) id: SurfaceId,
    pub(crate) label: String,
    pub(crate) scene: Scene,
    pub(crate) background: Color,
    pub(crate) child: Option<Element<Message>>,
}

impl<Message: 'static> WlWgpuWidget<Message> {
    pub(crate) fn handle_event(
        &mut self,
        msg: &mut Vec<Message>,
        shell: &mut Shell<Message>,
        kind: EventKind,
    ) -> Result<()> {
        match kind.clone() {
            EventKind::Draw => {
                self.draw(shell)?;
            }
            EventKind::Resize { width, height } => {
                shell.resize_surface(&self.id, width, height);

                self.draw(shell)?;
            }
            _ => {}
        }

        if let Some(child) = &mut self.child {
            child.handle_event(msg, kind)?;
        }

        Ok(())
    }

    pub(crate) fn draw(&mut self, shell: &mut Shell<Message>) -> Result<()> {
        self.scene.clear();

        let (width, height) = shell.size(&self.id)?;
        self.scene.fill(width, height, self.background);

        if let Some(child) = &mut self.child {
            child.draw(&mut self.scene)?;
        }

        shell.render(&self.id, &self.scene)?;

        Ok(())
    }

    pub(crate) fn new(root: RootWidget<Message>, shell: &mut Shell<Message>) -> Result<Self> {
        let (wl, wgpu, qh) = (&shell.wl, &mut shell.wgpu, &shell.qh);

        let wayland_surface = match root.window {
            true => WlSurfaceHandle::window(wl, &qh, root.title, root.width, root.height),
            false => WlSurfaceHandle::layer(
                wl,
                &qh,
                root.x,
                root.y,
                root.width,
                root.height,
                root.layer,
                root.anchor,
                root.keyboard,
            ),
        };

        let wgpu_surface = unsafe {
            pollster::block_on(
                wgpu.ctx.create_render_surface(
                    wgpu.ctx
                        .instance
                        .create_surface_unsafe(wayland_surface.surface_target(wl))?,
                    root.width,
                    root.height,
                    PresentMode::AutoVsync,
                ),
            )
            .map_err(Report::msg)?
        };

        let surface = Surface::new(wgpu_surface, wayland_surface);

        let id = wgpu.register_surface(surface)?;
        let label = root.label;
        let child = root.child;
        let background = root.background;
        let scene = Scene::new();

        Ok(Self {
            scene,
            label,
            id,
            background,
            child,
        })
    }
}
