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

delegate_compositor!(@<Message: 'static + Send + Sync> Client<Message>);

impl<Message: 'static + Send + Sync> CompositorHandler for Client<Message> {
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
        qh: &QueueHandle<Self>,
        surface: &WlSurface,
        _time: u32,
    ) {
        self.handle_event(WidgetId::Widget(surface.id()), WaylandWidgetEvent::Draw);

        surface.frame(qh, surface.clone());
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
