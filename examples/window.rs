use wui_rs::*;

#[tokio::main]
async fn main() -> Result<()> {
    use app::BasicApplication;

    Model::default().run().await
}

#[derive(Debug)]
enum Message {}

#[derive(Default)]
struct Model {}

impl Controller<Message> for Model {
    fn controller(&mut self, msg: Message) -> Task<Message> {
        println!("Received message: {:?}", msg);

        Task::none()
    }
}

impl View<Message> for Model {
    fn view(&self) -> impl IntoRootWidgets<Message> {
        root("window.1")
            .windowed()
            .background(palette::css::WHITE)
            .child(follower::<Message>())
    }
}
