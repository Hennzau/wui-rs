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
        self.client.send_no_result(Request::ForwardEvent {
            event: ViewEvent::ScaleFactorChanged(new_factor),
            id: Some(surface.id()),
        });
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        surface: &WlSurface,
        new_transform: Transform,
    ) {
        self.client.send_no_result(Request::ForwardEvent {
            event: ViewEvent::TransformChanged(new_transform),
            id: Some(surface.id()),
        });
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        surface: &WlSurface,
        _time: u32,
    ) {
        self.client.send_no_result(Request::ForwardEvent {
            event: ViewEvent::Frame,
            id: Some(surface.id()),
        });
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
