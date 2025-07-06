use std::time::Duration;

use tokio::task::JoinHandle;
use wui_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let (orchestrator, client) = Orchestrator::new()?;

    let client: JoinHandle<Result<()>> = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let view = client
            .query(Request::CreateViewLayer(ViewConfiguration {
                anchor: Anchor::TOP,
                size: (1920, 24),
                exclusive_zone: 24,
                ..Default::default()
            }))
            .await?;

        println!("Created view: {:?}", view);

        let view = client
            .query(Request::CreateViewLayer(ViewConfiguration {
                anchor: Anchor::BOTTOM,
                size: (1920, 24),
                exclusive_zone: 24,
                ..Default::default()
            }))
            .await?;

        println!("Created view: {:?}", view);

        let view = client
            .query(Request::CreateViewWindow(ViewConfiguration {
                decorations: WindowDecorations::ServerDefault,
                title: String::from("wui_rs"),
                app_id: String::from("io.github.wui_rs"),
                min_size: Some((1920 / 2, 1080 / 2)),
                ..Default::default()
            }))
            .await?;

        println!("Created view: {:?}", view);

        Ok(())
    });

    tokio::select! {
        result = orchestrator.run() => {
            client.await??;

            result
        }
    }
}
