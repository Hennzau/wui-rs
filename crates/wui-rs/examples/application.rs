use wui_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    Application::new(MyApp::default(), MyApp::view)?.run().await
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl MyApp {
    fn view(&self) -> Box<dyn ElementBuilder> {
        view().with_kind(ViewKind::Layer)
    }
}
