use vello::peniko::color::palette;
use wui_rs::*;

fn main() -> Result<()> {
    use app::BasicApplication;

    Model::run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    MouseEntered,
    MouseLeft,
}

#[derive(Default)]
struct Model {
    spawn: bool,
}

impl Controller<Message> for Model {
    fn controller(&mut self, msg: Message) -> impl IntoCommand<Message> {
        match msg {
            Message::MouseEntered => {
                self.spawn = true;
            }
            Message::MouseLeft => {
                self.spawn = false;
            }
        }

        Command::none()
    }
}

impl View<Message> for Model {
    fn view(&self) -> impl IntoRootWidgets<Message> {
        root("main").child(mouse(
            row()
                .child(mouse(
                    container(mouse(square().color(palette::css::RED)))
                        .color(palette::css::WHITE)
                        .margin(Insets::uniform(10.0)),
                ))
                .child(
                    mouse(square().color(palette::css::DARK_GREEN))
                        .on_enter(|| Message::MouseEntered)
                        .on_leave(|| Message::MouseLeft),
                )
                .child_if(self.spawn, circle().color(palette::css::BEIGE)),
        ))
    }
}
