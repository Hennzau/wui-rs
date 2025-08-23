use vello::peniko::color::palette;
use winit::event::MouseButton;
use wui_rs::*;

fn main() -> Result<()> {
    use app::BasicApplication;

    Model::run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    MouseEntered,
    MouseLeft,
    MousePressed,
    MouseReleased,
}

#[derive(Default)]
struct Model {
    spawn: bool,
}

impl Controller<Message> for Model {
    fn controller(&mut self, msg: Message) -> impl IntoCommand<Message> {
        match msg {
            Message::MouseEntered => {
                println!("Mouse Entered");
                self.spawn = true;
            }
            Message::MouseLeft => {
                self.spawn = false;
            }
            Message::MousePressed => {
                println!("Mouse Pressed");
                self.spawn = true;
            }
            Message::MouseReleased => {
                self.spawn = false;
            }
        }

        Command::none()
    }
}

impl View<Message> for Model {
    fn view(&self) -> impl IntoRootWidgets<Message> {
        root("main").child(mouse(
            "main",
            row()
                .child(mouse(
                    "container",
                    container(mouse("inner", square().color(palette::css::RED)))
                        .color(palette::css::WHITE)
                        .margin(Insets::uniform(10.0)),
                ))
                .child(
                    mouse("green", square().color(palette::css::DARK_GREEN))
                        .on_enter(|| Message::MouseEntered)
                        .on_leave(|| Message::MouseLeft)
                        .on_press(|button| {
                            if button.mouse_button() == MouseButton::Left {
                                Some(Message::MousePressed)
                            } else {
                                None
                            }
                        })
                        .on_release(|button| {
                            if button.mouse_button() == MouseButton::Left {
                                Some(Message::MouseReleased)
                            } else {
                                None
                            }
                        }),
                )
                .child_if(self.spawn, circle().color(palette::css::BEIGE)),
        ))
    }
}
