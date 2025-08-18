use smithay_client_toolkit::{
    compositor::CompositorState,
    reexports::client::{
        Connection, EventQueue, QueueHandle,
        globals::{GlobalList, registry_queue_init},
    },
    shell::{wlr_layer::LayerShell, xdg::XdgShell},
};

use crate::*;

pub struct Wl {
    pub(crate) xdg_shell: XdgShell,
    pub(crate) layer_shell: LayerShell,

    pub(crate) compositor_state: CompositorState,

    pub(crate) qh: QueueHandle<Client>,
    pub(crate) globals: GlobalList,

    pub(crate) connection: Connection,
}

impl Wl {
    pub(crate) fn new() -> Result<(Self, EventQueue<Client>)> {
        let connection = Connection::connect_to_env()?;

        let (globals, event_queue) = registry_queue_init::<Client>(&connection)?;
        let qh = event_queue.handle();

        let compositor_state = CompositorState::bind(&globals, &qh)?;
        let xdg_shell = XdgShell::bind(&globals, &qh)?;
        let layer_shell = LayerShell::bind(&globals, &qh)?;

        let wl = Wl {
            xdg_shell,
            layer_shell,
            compositor_state,
            qh,
            globals,
            connection,
        };

        Ok((wl, event_queue))
    }
}
