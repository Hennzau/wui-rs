use wui_view::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let (orchestrator, backend, _client) = Orchestrator::new()?;

    tokio::select! {
        result = orchestrator.run(backend) => {
            result
        }
    }
}
