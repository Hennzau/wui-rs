use std::{ptr::NonNull, sync::Arc};

use raw_window_handle::{
    RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
};
use tokio::sync::oneshot::Sender;
use wayland_client::{Connection, Proxy, protocol::wl_surface::WlSurface};
use wgpu::{
    Adapter, Device, Instance, PowerPreference, Queue, RequestAdapterOptions, Surface,
    SurfaceTargetUnsafe,
};

use crate::prelude::*;

pub(crate) async fn create_wgpu_primitives(
    instance: &Instance,
    connection: &Connection,
    surface: &WlSurface,
) -> Result<(Arc<Surface<'static>>, Arc<Adapter>, Arc<Device>, Arc<Queue>)> {
    let raw_display_handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
        NonNull::new(connection.backend().display_ptr() as *mut _).unwrap(),
    ));

    let raw_window_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(
        NonNull::new(surface.id().as_ptr() as *mut _).unwrap(),
    ));

    let surface = unsafe {
        instance
            .create_surface_unsafe(SurfaceTargetUnsafe::RawHandle {
                raw_display_handle,
                raw_window_handle,
            })
            .unwrap()
    };

    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            compatible_surface: Some(&surface),
            power_preference: PowerPreference::LowPower,
            ..Default::default()
        })
        .await?;

    let (device, queue) = adapter.request_device(&Default::default()).await?;

    Ok((
        Arc::new(surface),
        Arc::new(adapter),
        Arc::new(device),
        Arc::new(queue),
    ))
}

pub(crate) async fn handle_create<Message: 'static + Send + Sync>(
    views: Views<Message>,

    instance: &Instance,
    protocol: &WaylandProtocol<Message>,

    app_views: &mut Views<Message>,
    response: Option<Sender<Response>>,
) -> Result<()> {
    if let Err(e) = app_views.merge(views, instance, protocol).await {
        return send_response::<Message>(
            response,
            Response::Failed(format!("Failed to create views: {}", e)),
        );
    }

    send_response::<Message>(response, Response::Success)
}
