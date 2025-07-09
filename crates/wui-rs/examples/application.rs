use wui_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    Application::new(MyApp::default(), MyApp::views)?
        .run()
        .await
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl MyApp {
    fn views(&self) -> ViewsBuilder {
        vec![
            view()
                .with_configuration(ViewConfiguration {
                    anchor: Anchor::TOP,
                    namespace: String::from("Top bar"),
                    ..Default::default()
                })
                .with_child(rect()),
            view()
                .with_configuration(ViewConfiguration {
                    anchor: Anchor::BOTTOM,
                    namespace: String::from("Bottom bar"),
                    ..Default::default()
                })
                .with_child(rect()),
        ]
    }
}
