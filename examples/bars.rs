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
        vec![
            root("bar.top")
                .size((1920, 24).into())
                .background(palette::css::RED)
                .anchor(Anchor::TOP)
                .exclusive_zone(24),
            root("bar.bottom")
                .size((1920, 24).into())
                .background(palette::css::BLUE)
                .anchor(Anchor::BOTTOM)
                .exclusive_zone(24),
        ]
    }
}
