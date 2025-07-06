use ::wgpu::Instance;
use std::collections::HashMap;
use wayland_backend::client::ObjectId;

use crate::prelude::*;

pub mod channel;
pub use channel::*;

pub(crate) mod wayland;
pub(crate) use wayland::*;

pub(crate) mod wgpu;

use tokio::task::JoinHandle;

pub(crate) mod handler;

pub struct Orchestrator {
    pub(crate) inner: OrchestratorInner,
    pub(crate) backend: WaylandBackend,
}

impl Orchestrator {
    pub fn new() -> Result<(Self, Client)> {
        let (inner, backend, client) = OrchestratorInner::new()?;

        Ok((Self { inner, backend }, client))
    }

    pub async fn run(self) -> Result<()> {
        self.inner.run(self.backend).await
    }
}

pub(crate) struct OrchestratorInner {
    pub(crate) server: Server,
    pub(crate) protocol: WaylandProtocol,
    pub(crate) instance: Instance,

    pub(crate) client: Client,

    pub(crate) views: HashMap<ObjectId, View>,
}

impl OrchestratorInner {
    pub(crate) fn new() -> Result<(Self, WaylandBackend, Client)> {
        let (client, server) = Client::new();
        let (backend, protocol) = WaylandBackend::new(client.clone())?;
        let instance = Instance::default();

        Ok((
            Self {
                server,
                protocol,
                instance,

                client: client.clone(),

                views: HashMap::new(),
            },
            backend,
            client,
        ))
    }

    pub(crate) async fn run(mut self, backend: WaylandBackend) -> Result<()> {
        let server: JoinHandle<Result<()>> = tokio::task::spawn(async move {
            while let Ok(query) = self.server.recv().await {
                self.handle_query(query).await?;
            }

            Ok(())
        });

        let backend: JoinHandle<Result<()>> =
            tokio::task::spawn(async move { backend.run().await });

        tokio::select! {
            result = server => {
                result?
            },
            result = backend => {
                result?
            }
        }
    }
}
