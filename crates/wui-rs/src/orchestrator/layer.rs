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

delegate_layer!(State);

impl LayerShellHandler for State {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, layer: &LayerSurface) {
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::Closed,
            view_id: Some(layer.wl_surface().id()),
        }) {
            eprintln!("Failed to send layer surface closed event: {}", e);
        }
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::Configure(WaylandViewConfigure::LayerSurface(configure)),
            view_id: Some(layer.wl_surface().id()),
        }) {
            eprintln!("Failed to send layer surface configure event: {}", e);
        }
    }
}
