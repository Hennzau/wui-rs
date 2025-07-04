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
    fn request_close(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &Window) {}

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        window: &Window,
        _configure: WindowConfigure,
        _serial: u32,
    ) {
        println!("Configuring: {:?}", window.wl_surface().id());
    }
}
