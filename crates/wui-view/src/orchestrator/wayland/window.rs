use smithay_client_toolkit::{
    delegate_xdg_shell, delegate_xdg_window,
    shell::{
        WaylandSurface,
        xdg::window::{Window, WindowConfigure, WindowHandler},
    },
};
use wayland_client::{Connection, Proxy, QueueHandle};

use crate::prelude::*;

delegate_xdg_shell!(State);
delegate_xdg_window!(State);

impl WindowHandler for State {
    fn request_close(&mut self, _: &Connection, _: &QueueHandle<Self>, window: &Window) {
        self.client.send_no_result(Request::ForwardEvent {
            event: ViewEvent::Closed,
            id: Some(window.wl_surface().id()),
        });
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        window: &Window,
        configure: WindowConfigure,
        _serial: u32,
    ) {
        self.client.send_no_result(Request::ForwardEvent {
            event: ViewEvent::Configure(ViewConfigure::Window(configure)),
            id: Some(window.wl_surface().id()),
        });
    }
}
