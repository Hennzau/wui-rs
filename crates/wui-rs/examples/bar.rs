use smithay_client_toolkit::shell::xdg::window::WindowDecorations;
use wui_rs::prelude::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mut orchestrator = Orchestrator::new()?;

    let _view = orchestrator
        .create_layer(ViewConfiguration {
            anchor: Anchor::TOP,
            size: (1920, 24),
            exclusive_zone: 24,
            ..Default::default()
        })
        .await?;

    let _view = orchestrator
        .create_layer(ViewConfiguration {
            anchor: Anchor::BOTTOM,
            size: (1920, 24),
            exclusive_zone: 24,
            ..Default::default()
        })
        .await?;

    // let _view = orchestrator
    //     .create_window(ViewConfiguration {
    //         decorations: WindowDecorations::ServerDefault,
    //         title: String::from("wui_rs"),
    //         app_id: String::from("io.github.wui_rs"),
    //         min_size: Some((1920 / 2, 1080 / 2)),
    //         ..Default::default()
    //     })
    //     .await?;

    orchestrator.run().await
}
