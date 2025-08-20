use wui_rs::*;

fn main() -> Result<()> {
    use app::BasicApplication;

    Model::run()
}

enum Message {}

#[derive(Default)]
struct Model {}

impl Controller<Message> for Model {
    fn controller(&mut self, _: Message) -> impl IntoTask<Message> {
        Task::none()
    }
}

impl View<Message> for Model {
    fn view(&self) -> impl IntoRootWidgets<Message> {
        root("main").child(follower::<Message>())
    }
}
