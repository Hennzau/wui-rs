use std::ptr::NonNull;

use raw_window_handle::{
    RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
};
use smithay_client_toolkit::{
    reexports::client::{Proxy, QueueHandle},
    shell::{
        WaylandSurface,
        wlr_layer::LayerSurface,
        xdg::{
            XdgSurface,
            window::{Window, WindowDecorations},
        },
    },
};
use vello::wgpu::SurfaceTargetUnsafe;

use crate::*;

pub(crate) enum WlSurfaceHandle {
    Layer(LayerSurface),
    Window(Window),
}

impl WlSurfaceHandle {
    pub(crate) fn request_redraw<Message: 'static>(&self, qh: &QueueHandle<Client<Message>>) {
        match self {
            WlSurfaceHandle::Layer(layer) => {
                layer.wl_surface().frame(qh, layer.wl_surface().clone());
                layer.wl_surface().commit();
            }
            WlSurfaceHandle::Window(window) => {
                window.wl_surface().frame(qh, window.wl_surface().clone());
                window.wl_surface().commit();
            }
        }
    }

    pub(crate) fn id(&self) -> SurfaceId {
        match self {
            WlSurfaceHandle::Layer(layer) => SurfaceId(layer.wl_surface().id()),
            WlSurfaceHandle::Window(window) => SurfaceId(window.wl_surface().id()),
        }
    }

    pub(crate) fn destroy(&self) {
        match self {
            WlSurfaceHandle::Layer(layer) => {
                layer.wl_surface().destroy();
            }
            WlSurfaceHandle::Window(window) => {
                window.wl_surface().destroy();
            }
        }
    }

    pub(crate) fn window<Message: 'static>(
        wl: &Wl,
        qh: &QueueHandle<Client<Message>>,
        title: String,
        width: u32,
        height: u32,
    ) -> Self {
        let wl_surface = wl.compositor_state.create_surface(qh);

        let window = wl
            .xdg_shell
            .create_window(wl_surface, WindowDecorations::ServerDefault, qh);

        window.set_app_id(title.clone());
        window.set_title(title);

        window
            .xdg_surface()
            .set_window_geometry(0, 0, width as i32, height as i32);

        window.commit();

        WlSurfaceHandle::Window(window)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn layer<Message: 'static>(
        wl: &Wl,
        qh: &QueueHandle<Client<Message>>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        layer: Layer,
        anchor: Anchor,
        keyboard: KeyboardInteractivity,
    ) -> Self {
        let wl_surface = wl.compositor_state.create_surface(qh);

        let layer = wl.layer_shell.create_layer_surface(
            qh,
            wl_surface,
            match layer {
                Layer::Top => smithay_client_toolkit::shell::wlr_layer::Layer::Top,
                Layer::Background => smithay_client_toolkit::shell::wlr_layer::Layer::Background,
            },
            None::<String>,
            None,
        );

        match anchor {
            Anchor::None => {}
            Anchor::Top(exclusive) => {
                layer.set_anchor(smithay_client_toolkit::shell::wlr_layer::Anchor::TOP);
                layer.set_exclusive_zone(exclusive as i32);
            }
            Anchor::Left(exclusive) => {
                layer.set_anchor(smithay_client_toolkit::shell::wlr_layer::Anchor::LEFT);
                layer.set_exclusive_zone(exclusive as i32);
            }
            Anchor::Right(exclusive) => {
                layer.set_anchor(smithay_client_toolkit::shell::wlr_layer::Anchor::RIGHT);
                layer.set_exclusive_zone(exclusive as i32);
            }
            Anchor::Bottom(exclusive) => {
                layer.set_anchor(smithay_client_toolkit::shell::wlr_layer::Anchor::BOTTOM);
                layer.set_exclusive_zone(exclusive as i32);
            }
        }

        layer.set_keyboard_interactivity(keyboard.into());
        layer.set_size(width, height);
        layer.set_margin(y as i32, 0, 0, x as i32);

        layer.commit();

        WlSurfaceHandle::Layer(layer)
    }

    pub(crate) fn surface_target(&self, wl: &Wl) -> SurfaceTargetUnsafe {
        let raw_display_handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
            NonNull::new(wl.connection.backend().display_ptr() as *mut _).unwrap(),
        ));

        let raw_window_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(
            NonNull::new(self.id().0.as_ptr() as *mut _).unwrap(),
        ));

        SurfaceTargetUnsafe::RawHandle {
            raw_display_handle,
            raw_window_handle,
        }
    }
}
