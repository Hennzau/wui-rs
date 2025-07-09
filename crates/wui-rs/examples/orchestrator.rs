use tokio::task::JoinHandle;

use wui_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let (orchestrator, client) = Orchestrator::new()?;

    let client: JoinHandle<Result<()>> = tokio::spawn(async move {
        let view = client
            .query(Request::CreateViewLayer(ViewConfiguration {
                namespace: String::from("Top bar"),
                anchor: Anchor::TOP,
                size: (1920, 24),
                exclusive_zone: 24,
                ..Default::default()
            }))
            .await?;

        println!("Created view: {:?}", view);

        let view = client
            .query(Request::CreateViewLayer(ViewConfiguration {
                namespace: String::from("Bottom bar"),
                anchor: Anchor::BOTTOM,
                size: (1920, 24),
                exclusive_zone: 24,
                ..Default::default()
            }))
            .await?;

        println!("Created view: {:?}", view);

        let view = client
            .query(Request::CreateViewWindow(ViewConfiguration {
                namespace: String::from("Settings"),
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
        _ = orchestrator.run() => {
            println!("Orchestrator stopped");
            client.await??;
        }
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C");
            client.await??;
        }
    }

    Ok(())
}
