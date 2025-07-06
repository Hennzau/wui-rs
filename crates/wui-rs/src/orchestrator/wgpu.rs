use std::ptr::NonNull;

use ::wgpu::{Adapter, Device, Queue, Surface};
use ::wgpu::{PowerPreference, RequestAdapterOptions, SurfaceTargetUnsafe};
use raw_window_handle::{
    RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
};

use wayland_client::Proxy;
use wayland_client::protocol::wl_surface::WlSurface;

use crate::prelude::*;

impl OrchestratorInner {
    pub(crate) async fn create_wgpu_primitives(
        &self,
        surface: &WlSurface,
    ) -> Result<(Surface<'static>, Adapter, Device, Queue)> {
        let raw_display_handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
            NonNull::new(self.protocol.connection.backend().display_ptr() as *mut _).unwrap(),
        ));

        let raw_window_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(
            NonNull::new(surface.id().as_ptr() as *mut _).unwrap(),
        ));

        let surface = unsafe {
            self.instance
                .create_surface_unsafe(SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle,
                    raw_window_handle,
                })
                .unwrap()
        };

        let adapter = self
            .instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: PowerPreference::LowPower,
                ..Default::default()
            })
            .await?;

        let (device, queue) = adapter.request_device(&Default::default()).await?;

        Ok((surface, adapter, device, queue))
    }
}
