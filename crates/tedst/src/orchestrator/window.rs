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
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::Closed,
            view_id: Some(window.wl_surface().id()),
        }) {
            eprintln!("Failed to send request close event: {}", e);
        }
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        window: &Window,
        configure: WindowConfigure,
        _serial: u32,
    ) {
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::Configure(WaylandViewConfigure::Window(configure)),
            view_id: Some(window.wl_surface().id()),
        }) {
            eprintln!("Failed to send window configure event: {}", e);
        }
    }
}
