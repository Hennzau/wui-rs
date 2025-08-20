use wui_rs::*;

fn main() -> Result<()> {
    use app::BasicApplication;

    Model::run()
}

enum Message {}

#[derive(Default)]
struct Model {}

impl Controller<Message> for Model {
    fn controller(&mut self, _: Message) -> impl IntoCommand<Message> {
        Command::none()
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
