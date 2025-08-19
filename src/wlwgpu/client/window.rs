use smithay_client_toolkit::{
    delegate_xdg_shell, delegate_xdg_window,
    reexports::client::{Connection, Proxy, QueueHandle},
    shell::{
        WaylandSurface,
        xdg::{
            XdgSurface,
            window::{Window, WindowConfigure, WindowHandler},
        },
    },
};

use crate::*;

delegate_xdg_shell!(@<Message: 'static> Client<Message>);
delegate_xdg_window!(@<Message: 'static> Client<Message>);

impl<Message: 'static> WindowHandler for Client<Message> {
    fn request_close(&mut self, _: &Connection, _: &QueueHandle<Self>, window: &Window) {
        self.handle(Some(window.wl_surface().id().into()), EventKind::Close);
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        window: &Window,
        configure: WindowConfigure,
        _serial: u32,
    ) {
        let (width, height) = (
            configure.new_size.0.map(|n| n.get()).unwrap_or(256),
            configure.new_size.1.map(|n| n.get()).unwrap_or(256),
        );

        window
            .xdg_surface()
            .set_window_geometry(0, 0, width as i32, height as i32);

        window.commit();

        self.handle(
            Some(window.wl_surface().id().into()),
            EventKind::Resize { width, height },
        );

        self.handle(Some(window.wl_surface().id().into()), EventKind::Draw);
    }
}
