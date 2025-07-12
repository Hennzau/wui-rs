use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use wgpu::Instance;

use crate::prelude::*;

pub(crate) struct ApplicationInner<Message: 'static + Send + Sync> {
    pub(crate) instance: Instance,
    pub(crate) backend: WaylandBackend<Message>,
    pub(crate) protocol: WaylandProtocol<Message>,

    pub(crate) server: Server<Message>,
    pub(crate) client: Client<Message>,

    pub(crate) sender: UnboundedSender<Message>,
}

impl<Message: 'static + Send + Sync> ApplicationInner<Message> {
    pub(crate) fn new() -> Result<(UnboundedReceiver<Message>, Self)> {
        let (client, server) = Client::new();
        let instance = Instance::default();
        let (backend, protocol) = WaylandBackend::new(client.clone())?;
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<Message>();

        Ok((
            receiver,
            Self {
                instance,
                backend,
                protocol,
                server,
                client,
                sender,
            },
        ))
    }

    pub(crate) async fn run(mut self) -> Result<()> {
        let server: JoinHandle<Result<()>> = tokio::spawn(async move {
            while let Ok(_query) = self.server.recv().await {
                // Handle the query here
            }

            Ok(())
        });

        let backend: JoinHandle<Result<()>> = tokio::spawn(async move { self.backend.run().await });

        tokio::select! {
            res = server => res?,
            res = backend => res?,
        }
    }
}
