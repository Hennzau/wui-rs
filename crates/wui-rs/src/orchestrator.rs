use std::sync::{Arc, atomic::AtomicBool};

use ::wgpu::Instance;

use crate::prelude::*;

pub mod channel;
pub use channel::*;

pub(crate) mod wayland;
pub(crate) use wayland::*;

pub(crate) mod wgpu;

pub(crate) mod views;
pub(crate) use views::*;

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

    pub fn client(&self) -> Client {
        self.inner.client()
    }

    pub async fn run(self, running: Arc<AtomicBool>) -> Result<()> {
        self.inner.run(self.backend, running).await
    }
}

pub(crate) struct OrchestratorInner {
    pub(crate) server: Server,
    pub(crate) protocol: WaylandProtocol,
    pub(crate) instance: Instance,

    pub(crate) client: Client,

    pub(crate) views: Views,
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

                views: Views::new(),
            },
            backend,
            client,
        ))
    }

    pub(crate) fn client(&self) -> Client {
        self.client.clone()
    }

    pub(crate) async fn run(
        mut self,
        backend: WaylandBackend,
        running: Arc<AtomicBool>,
    ) -> Result<()> {
        let server: JoinHandle<Result<()>> = tokio::task::spawn(async move {
            while let Ok(query) = self.server.recv().await {
                self.handle_query(query).await?;
            }

            Ok(())
        });

        let backend: JoinHandle<Result<()>> =
            tokio::task::spawn(async move { backend.run(running).await });

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
