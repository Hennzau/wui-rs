use smithay_client_toolkit::{
    compositor::CompositorState,
    delegate_registry,
    output::OutputState,
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::SeatState,
    shell::{wlr_layer::LayerShell, xdg::XdgShell},
};

use wayland_backend::client::ObjectId;
use wayland_client::{
    Connection, QueueHandle,
    globals::GlobalList,
    protocol::{wl_keyboard::WlKeyboard, wl_pointer::WlPointer},
};
use wgpu::Instance;

use crate::wayland::{WaylandElements, WaylandWidgetEvent};

use crate::prelude::*;

mod compositor;
mod keyboard;
mod layer;
mod output;
mod pointer;
mod seat;
mod window;

pub(crate) struct Protocol<Message> {
    pub(crate) connection: Connection,
    pub(crate) compositor_state: CompositorState,
    pub(crate) instance: Instance,

    pub(crate) xdg_shell: XdgShell,
    pub(crate) layer_shell: LayerShell,
    pub(crate) qh: QueueHandle<Client<Message>>,
}

impl<Message> Protocol<Message> {
    pub fn new(
        connection: Connection,
        compositor_state: CompositorState,
        instance: Instance,

        xdg_shell: XdgShell,
        layer_shell: LayerShell,
        qh: QueueHandle<Client<Message>>,
    ) -> Self {
        Self {
            connection,
            compositor_state,
            instance,

            xdg_shell,
            layer_shell,
            qh,
        }
    }
}

pub(crate) enum WidgetId {
    AllWidgets,
    Widget(ObjectId),
}

pub(crate) struct Client<Message> {
    pub(crate) msg: Sender<Message>,
    pub(crate) elements: WaylandElements<Message>,

    pub(crate) renderer: Renderer,

    pub(crate) registry_state: RegistryState,
    pub(crate) seat_state: SeatState,
    pub(crate) output_state: OutputState,

    pub(crate) keyboard: Option<WlKeyboard>,
    pub(crate) pointer: Option<WlPointer>,
}

impl<Message: 'static + Send + Sync> Client<Message> {
    pub(crate) fn new(
        msg: Sender<Message>,
        globals: &GlobalList,
        qh: &QueueHandle<Self>,
        renderer: Renderer,
    ) -> Self {
        let registry_state = RegistryState::new(globals);
        let seat_state = SeatState::new(globals, qh);
        let output_state = OutputState::new(globals, qh);

        let elements = WaylandElements::new();

        let (keyboard, pointer) = (None, None);

        Self {
            msg,
            elements,

            renderer,

            registry_state,
            seat_state,
            output_state,
            keyboard,
            pointer,
        }
    }

    pub(crate) fn add(&mut self, protocol: &Protocol<Message>, element: Element<Message>) {
        let label = element.label().unwrap_or_default();

        let element = match self.elements.extract(&label) {
            Some(existing) => {
                let surface = existing.surface;
                surface.configure_wayland_surface(&element);

                WaylandElement::with_surface(surface, element)
            }
            None => WaylandElement::new(protocol, element),
        };

        self.elements.add(element);
    }

    pub(crate) fn destroy(&mut self, label: &Label) {
        self.elements.destroy(label);
    }

    pub(crate) fn throw_event(&mut self, id: WidgetId, event: WaylandWidgetEvent) {
        self.elements
            .on_event(id, event, self.msg.clone(), &mut self.renderer)
            .unwrap_or_else(|e| {
                tracing::warn!("Error while processing event: {:?}", e);
            });
    }
}

delegate_registry!(@<Message: 'static + Send + Sync> Client<Message>);

impl<Message: 'static + Send + Sync> ProvidesRegistryState for Client<Message> {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
