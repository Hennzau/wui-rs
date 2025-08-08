use crate::prelude::*;

use smithay_client_toolkit::{
    delegate_layer,
    reexports::client::{Connection, QueueHandle},
    shell::{
        WaylandSurface,
        wlr_layer::{LayerShellHandler, LayerSurface, LayerSurfaceConfigure},
    },
};
use wayland_client::Proxy;

delegate_layer!(@<Message: 'static + Send + Sync> Client<Message>);

impl<Message: 'static + Send + Sync> LayerShellHandler for Client<Message> {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, layer: &LayerSurface) {
        self.handle_event(
            WidgetId::Widget(layer.wl_surface().id()),
            WaylandWidgetEvent::Close,
        );
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        self.handle_event(
            WidgetId::Widget(layer.wl_surface().id()),
            WaylandWidgetEvent::Configure {
                width: configure.new_size.0,
                height: configure.new_size.1,
            },
        );

        self.handle_event(
            WidgetId::Widget(layer.wl_surface().id()),
            WaylandWidgetEvent::Draw,
        );
    }
}
