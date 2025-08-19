use smithay_client_toolkit::reexports::client::QueueHandle;

use crate::*;

pub struct Shell<Message> {
    pub(crate) qh: QueueHandle<Client<Message>>,

    pub(crate) wgpu: Wgpu,
    pub(crate) wl: Wl,
}

impl<Message: 'static> Shell<Message> {
    pub(crate) fn new(qh: QueueHandle<Client<Message>>, wl: Wl, wgpu: Wgpu) -> Self {
        Self { qh, wl, wgpu }
    }

    pub fn destroy_surface(&mut self, id: &SurfaceId) {
        self.wgpu.destroy_surface(id);
    }

    pub fn resize_surface(&mut self, id: &SurfaceId, width: u32, height: u32) {
        self.wgpu.resize_surface(id, width, height);
    }

    pub fn render(&mut self, id: &SurfaceId, scene: &Scene) -> Result<()> {
        self.wgpu.render(id, scene)
    }

    pub fn surfaces(&self) -> usize {
        self.wgpu.surfaces.len()
    }

    pub fn exists(&self, id: &SurfaceId) -> bool {
        self.wgpu.surfaces.contains_key(id)
    }

    pub fn size(&self, id: &SurfaceId) -> Result<(u32, u32)> {
        self.wgpu.size(id)
    }

    pub fn request_redraw(&self, id: &SurfaceId) {
        if let Some(surface) = self.wgpu.surfaces.get(id) {
            surface.wayland.request_redraw(&self.qh);
        }
    }
}
