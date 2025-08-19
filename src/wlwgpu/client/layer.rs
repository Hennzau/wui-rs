use crate::*;

use smithay_client_toolkit::{
    delegate_layer,
    reexports::client::{Connection, Proxy, QueueHandle},
    shell::{
        WaylandSurface,
        wlr_layer::{LayerShellHandler, LayerSurface, LayerSurfaceConfigure},
    },
};

delegate_layer!(@<Message: 'static> Client<Message>);

impl<Message: 'static> LayerShellHandler for Client<Message> {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, layer: &LayerSurface) {
        self.handle(Some(layer.wl_surface().id().into()), EventKind::Close);
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        layer.set_size(configure.new_size.0, configure.new_size.1);
        layer.commit();

        self.handle(
            Some(layer.wl_surface().id().into()),
            EventKind::Resize {
                width: configure.new_size.0,
                height: configure.new_size.1,
            },
        );

        self.handle(Some(layer.wl_surface().id().into()), EventKind::Draw);
    }
}
