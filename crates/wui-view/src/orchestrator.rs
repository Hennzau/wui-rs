use crate::prelude::*;

pub mod channel;
pub use channel::*;

pub(crate) mod wayland;
pub(crate) use wayland::*;

use tokio::task::JoinHandle;

pub(crate) mod handler;

pub struct Orchestrator {
    pub(crate) server: Server,
    pub(crate) protocol: WaylandProtocol,
}

impl Orchestrator {
    pub fn new() -> Result<(Self, WaylandBackend, Client)> {
        let (client, server) = Client::new();
        let (backend, protocol) = WaylandBackend::new(client.clone())?;

        Ok((Self { server, protocol }, backend, client))
    }

    pub async fn run(mut self, backend: WaylandBackend) -> Result<()> {
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
