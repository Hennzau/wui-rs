use std::num::NonZeroU32;

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
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _layer: &LayerSurface) {}

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        layer: &LayerSurface,
        _configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        println!("Configuring: {}", layer.wl_surface().id());
        // let width = NonZeroU32::new(configure.new_size.0).map_or(256, NonZeroU32::get);
        // let height = NonZeroU32::new(configure.new_size.1).map_or(256, NonZeroU32::get);

        // let adapter = &self.adapter;
        // let surface = &self.surface;
        // // let device = &self.device;
        // // let queue = &self.queue;

        // let cap = surface.get_capabilities(&adapter);
        // let surface_config = wgpu::SurfaceConfiguration {
        //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     format: cap.formats[0],
        //     view_formats: vec![cap.formats[0]],
        //     alpha_mode: wgpu::CompositeAlphaMode::Auto,
        //     width: self.width,
        //     height: self.height,
        //     desired_maximum_frame_latency: 2,
        //     // Wayland is inherently a mailbox system.
        //     present_mode: wgpu::PresentMode::Mailbox,
        // };

        // surface.configure(&self.device, &surface_config);
    }
}
