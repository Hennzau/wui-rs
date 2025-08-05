use std::time::Duration;

use smithay_client_toolkit::{
    compositor::CompositorState,
    shell::{wlr_layer::LayerShell, xdg::XdgShell},
};
use wayland_client::{Connection, EventQueue, globals::registry_queue_init};
use wgpu::{Instance, PowerPreference, RequestAdapterOptions};

use crate::prelude::*;

pub enum Request<Message> {
    Spawn(Element<Message>),
    Destroy(Label),
}

pub struct Backend<Message> {
    pub(crate) tx: Sender<Request<Message>>,
    pub(crate) rx: Receiver<Request<Message>>,

    pub(crate) client: Client<Message>,

    pub(crate) protocol: Protocol<Message>,

    pub(crate) event_queue: EventQueue<Client<Message>>,
}

impl<Message: 'static + Send + Sync> Backend<Message> {
    pub async fn new(msg: Sender<Message>) -> Result<Self> {
        let (tx, rx) = channel();

        let connection = Connection::connect_to_env()?;

        let (globals, event_queue) = registry_queue_init::<Client<Message>>(&connection)?;
        let qh = event_queue.handle();

        let compositor_state = CompositorState::bind(&globals, &qh)?;
        let xdg_shell = XdgShell::bind(&globals, &qh)?;
        let layer_shell = LayerShell::bind(&globals, &qh)?;

        let instance = Instance::default();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::LowPower,
                ..Default::default()
            })
            .await?;

        let (device, queue) = adapter.request_device(&Default::default()).await?;
        let renderer = Renderer::new(adapter, device, queue);

        let client = Client::new(msg, &globals, &qh, renderer);
        let protocol = Protocol::new(
            connection,
            compositor_state,
            instance,
            xdg_shell,
            layer_shell,
            event_queue.handle(),
        );

        Ok(Self {
            tx,
            rx,
            client,
            event_queue,
            protocol,
        })
    }

    pub fn tx(&self) -> Sender<Request<Message>> {
        self.tx.clone()
    }
}

impl<Message: 'static + Send + Sync> Backend<Message> {
    pub async fn run(mut self) -> Result<()> {
        tracing::info!("Backend started");

        loop {
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_millis(16)) => {
                    self.event_queue.flush()?;

                    if let Some(guard) = self.event_queue.prepare_read() {
                        if let Err(e) = guard.read_without_dispatch() {
                            eprintln!("Error reading events: {:?}", e);
                        }
                    }

                    self.event_queue.dispatch_pending(&mut self.client).unwrap();
                }
                Ok(request) = self.rx.recv() => {
                    match request {
                        Request::Spawn(element) => {
                            for element in element.into_list() {
                                self.client.add(&self.protocol, element);
                            }
                        }
                        Request::Destroy(label) => {
                            tracing::debug!("Destroying element with label: {}", label);

                            self.client.destroy(&label);
                        }
                    }
                }
            }
        }
    }
}
