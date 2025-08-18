use vello::util::RenderSurface;
use wayland_backend::client::ObjectId;

mod builder;
pub use builder::*;

mod wl;
pub(crate) use wl::*;

pub enum KeyboardInteractivity {
    None,
    OnDemand,
    Exclusive,
}

pub enum Anchor {
    None,
    Top(u32),
    Left(u32),
    Right(u32),
    Bottom(u32),
}

pub enum Layer {
    Top,
    Background,
}

impl From<KeyboardInteractivity>
    for smithay_client_toolkit::shell::wlr_layer::KeyboardInteractivity
{
    fn from(interactivity: KeyboardInteractivity) -> Self {
        match interactivity {
            KeyboardInteractivity::None => {
                smithay_client_toolkit::shell::wlr_layer::KeyboardInteractivity::None
            }
            KeyboardInteractivity::OnDemand => {
                smithay_client_toolkit::shell::wlr_layer::KeyboardInteractivity::OnDemand
            }
            KeyboardInteractivity::Exclusive => {
                smithay_client_toolkit::shell::wlr_layer::KeyboardInteractivity::Exclusive
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SurfaceId(pub(crate) ObjectId);

impl From<ObjectId> for SurfaceId {
    fn from(id: ObjectId) -> Self {
        SurfaceId(id)
    }
}

pub struct Surface {
    pub(crate) wgpu: RenderSurface<'static>,
    pub(crate) wayland: WlSurfaceHandle,
}

impl Surface {
    pub(crate) fn new(wgpu: RenderSurface<'static>, wayland: WlSurfaceHandle) -> Self {
        Surface { wgpu, wayland }
    }

    pub(crate) fn dev_id(&self) -> usize {
        self.wgpu.dev_id
    }

    pub(crate) fn id(&self) -> SurfaceId {
        self.wayland.id()
    }

    pub(crate) fn destroy(&self) {
        self.wayland.destroy();
    }
}
