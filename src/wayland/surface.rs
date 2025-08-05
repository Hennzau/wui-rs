use std::ptr::NonNull;

use crate::prelude::*;
use raw_window_handle::{
    RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle, WaylandWindowHandle,
};
use smithay_client_toolkit::{
    compositor::CompositorState,
    shell::{
        WaylandSurface,
        wlr_layer::{Anchor, KeyboardInteractivity, Layer, LayerShell, LayerSurface},
        xdg::{
            XdgShell,
            window::{Window, WindowDecorations},
        },
    },
};
use wayland_backend::client::ObjectId;
use wayland_client::{Proxy, QueueHandle};
use wgpu::SurfaceTargetUnsafe;

pub(crate) enum WlSurfaceHandle {
    Layer(LayerSurface),
    Window(Window),
}

impl WlSurfaceHandle {
    pub(crate) fn destroy(&self) {
        match self {
            Self::Layer(layer) => layer.wl_surface().destroy(),
            Self::Window(window) => window.wl_surface().destroy(),
        }
    }

    pub(crate) fn configure_wayland<Message: 'static + Send + Sync>(
        &self,
        element: &Element<Message>,
    ) {
        match self {
            Self::Layer(layer) => {
                let location = match element.display_mode() {
                    DisplayMode::Layered { location, .. } => location,
                    _ => {
                        panic!("Cannot configure a layer surface without a location");
                    }
                };

                Self::configure_layer(
                    layer,
                    match location.side {
                        Some(side) => match side {
                            Side::Top => Anchor::TOP,
                            Side::Left => Anchor::LEFT,
                            Side::Right => Anchor::RIGHT,
                            Side::Bottom => Anchor::BOTTOM,
                        },
                        None => Anchor::TOP | Anchor::LEFT,
                    },
                    match element.keyboard() {
                        KeyboardInteraction::None => KeyboardInteractivity::None,
                        KeyboardInteraction::Aware => KeyboardInteractivity::OnDemand,
                        KeyboardInteraction::Exclusive => KeyboardInteractivity::Exclusive,
                    },
                    (element.size().width, element.size().height),
                    location.exclusive,
                    (location.y as i32, 0, 0, location.x as i32),
                );
            }
            Self::Window(window) => {
                Self::configure_window(window, &element.label().unwrap_or_default(), None, None);
            }
        }
    }

    pub(crate) fn configure_layer(
        layer: &LayerSurface,

        anchor: Anchor,
        keyboard_interactivity: KeyboardInteractivity,
        size: (u32, u32),
        exclusive_zone: u32,
        margin: (i32, i32, i32, i32),
    ) {
        layer.set_anchor(anchor);
        layer.set_keyboard_interactivity(keyboard_interactivity);
        layer.set_size(size.0, size.1);
        layer.set_exclusive_zone(exclusive_zone as i32);
        layer.set_margin(margin.0, margin.1, margin.2, margin.3);

        layer.commit();
    }

    pub(crate) fn configure_window(
        window: &Window,
        label: &str,
        min_size: Option<(u32, u32)>,
        max_size: Option<(u32, u32)>,
    ) {
        window.set_title(label);
        window.set_app_id(label);
        window.set_min_size(min_size);
        window.set_max_size(max_size);

        window.commit();
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn layer<Message: 'static + Send + Sync>(
        compositor_state: &CompositorState,

        layer_shell: &LayerShell,
        qh: &QueueHandle<Client<Message>>,

        layer: Layer,
        label: String,
        anchor: Anchor,
        keyboard_interactivity: KeyboardInteractivity,
        size: (u32, u32),
        exclusive_zone: u32,
        margin: (i32, i32, i32, i32),
    ) -> Self {
        let wl_surface = compositor_state.create_surface(qh);

        let layer =
            layer_shell.create_layer_surface(qh, wl_surface, layer, Some(label.clone()), None);

        Self::configure_layer(
            &layer,
            anchor,
            keyboard_interactivity,
            size,
            exclusive_zone,
            margin,
        );

        Self::Layer(layer)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn window<Message: 'static + Send + Sync>(
        compositor_state: &CompositorState,

        xdg_shell: &XdgShell,
        qh: &QueueHandle<Client<Message>>,

        decorations: WindowDecorations,
        label: String,
        min_size: Option<(u32, u32)>,
        max_size: Option<(u32, u32)>,
    ) -> Self {
        let wl_surface = compositor_state.create_surface(qh);

        let window = xdg_shell.create_window(wl_surface, decorations, qh);

        Self::configure_window(&window, &label, min_size, max_size);

        Self::Window(window)
    }

    pub(crate) fn id(&self) -> ObjectId {
        match self {
            Self::Layer(layer) => layer.wl_surface().id(),
            Self::Window(window) => window.wl_surface().id(),
        }
    }
}

pub struct Surface {
    pub(crate) wgpu_surface: wgpu::Surface<'static>,
    pub(crate) wayland_surface: WlSurfaceHandle,
}

impl Surface {
    pub(crate) fn id(&self) -> ObjectId {
        self.wayland_surface.id()
    }

    pub(crate) fn destroy(&self) {
        self.wayland_surface.destroy();
    }

    pub(crate) fn configure_wayland_surface<Message: 'static + Send + Sync>(
        &self,
        element: &Element<Message>,
    ) {
        self.wayland_surface.configure_wayland(element);
    }

    pub(crate) fn new<Message: 'static + Send + Sync>(
        protocol: &Protocol<Message>,
        element: &Element<Message>,
    ) -> Self {
        let wayland_surface = match element.display_mode() {
            DisplayMode::Layered { location, kind } => WlSurfaceHandle::layer(
                &protocol.compositor_state,
                &protocol.layer_shell,
                &protocol.qh,
                match kind {
                    LayerKind::Top => Layer::Top,
                    LayerKind::Background => Layer::Background,
                },
                element.label().unwrap_or_default(),
                match location.side {
                    Some(side) => match side {
                        Side::Top => Anchor::TOP,
                        Side::Left => Anchor::LEFT,
                        Side::Right => Anchor::RIGHT,
                        Side::Bottom => Anchor::BOTTOM,
                    },
                    None => Anchor::TOP | Anchor::LEFT,
                },
                match element.keyboard() {
                    KeyboardInteraction::None => KeyboardInteractivity::None,
                    KeyboardInteraction::Aware => KeyboardInteractivity::OnDemand,
                    KeyboardInteraction::Exclusive => KeyboardInteractivity::Exclusive,
                },
                (element.size().width, element.size().height),
                location.exclusive,
                (location.y as i32, 0, 0, location.x as i32),
            ),
            DisplayMode::Windowed => WlSurfaceHandle::window(
                &protocol.compositor_state,
                &protocol.xdg_shell,
                &protocol.qh,
                WindowDecorations::ServerDefault,
                element.label().unwrap_or_default(),
                None,
                None,
            ),
        };

        let raw_display_handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
            NonNull::new(protocol.connection.backend().display_ptr() as *mut _).unwrap(),
        ));

        let raw_window_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(
            NonNull::new(wayland_surface.id().as_ptr() as *mut _).unwrap(),
        ));

        let wgpu_surface = unsafe {
            protocol
                .instance
                .create_surface_unsafe(SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle,
                    raw_window_handle,
                })
                .unwrap()
        };

        Self {
            wgpu_surface,
            wayland_surface,
        }
    }
}
