use crate::prelude::*;

use smithay_client_toolkit::{
    compositor::CompositorHandler,
    delegate_compositor,
    reexports::client::{
        Connection, QueueHandle,
        protocol::{
            wl_output::{Transform, WlOutput},
            wl_surface::WlSurface,
        },
    },
};
use wayland_client::Proxy;

delegate_compositor!(State);

impl CompositorHandler for State {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        surface: &WlSurface,
        new_factor: i32,
    ) {
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::ScaleFactorChanged(new_factor),
            view_id: Some(surface.id()),
        }) {
            eprintln!("Failed to send scale factor change event: {}", e);
        }
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        surface: &WlSurface,
        new_transform: Transform,
    ) {
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::TransformChanged(new_transform),
            view_id: Some(surface.id()),
        }) {
            eprintln!("Failed to send transform change event: {}", e);
        }
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        surface: &WlSurface,
        time: u32,
    ) {
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::Frame(time),
            view_id: Some(surface.id()),
        }) {
            eprintln!("Failed to send frame event: {}", e);
        }
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _output: &WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _output: &WlOutput,
    ) {
    }
}
