use crate::*;

use smithay_client_toolkit::{
    compositor::CompositorHandler,
    delegate_compositor,
    reexports::client::{
        Connection, Proxy, QueueHandle,
        protocol::{
            wl_output::{Transform, WlOutput},
            wl_surface::WlSurface,
        },
    },
};

delegate_compositor!(Client);

impl CompositorHandler for Client {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _new_factor: i32,
    ) {
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _new_transform: Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        surface: &WlSurface,
        _time: u32,
    ) {
        self.handle(Some(surface.id().into()), EventKind::Draw);
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
