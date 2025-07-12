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

delegate_layer!(@<Message: 'static + Send + Sync> State<Message>);

impl<Message: 'static + Send + Sync> LayerShellHandler for State<Message> {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, layer: &LayerSurface) {
        self.client.send_no_result(Request::Close {
            id: layer.wl_surface().id(),
        });
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        self.client.send_no_result(Request::Distribute {
            event: Event::Configure {
                width: configure.new_size.0,
                height: configure.new_size.1,
            },
            id: Some(layer.wl_surface().id()),
        });
    }
}
