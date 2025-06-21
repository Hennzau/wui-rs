use wui_rs::prelude::*;

fn main() -> Result<()> {
    application(State::default, State::update, vec![State::view])?
        .with_message(Message::OpenConnectionPanel)?
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenConnectionPanel,
    PasswordInput(String),
    ProcessPasswordFailed(String),
    Login,
}

pub struct State {
    connection_panel: bool,
    logged_in: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            connection_panel: false,
            logged_in: false,
        }
    }
}

impl State {
    pub fn update(&mut self, message: Message) -> Result<Tasks<Message>> {
        match message {
            Message::OpenConnectionPanel => {
                self.connection_panel = true;

                Ok(Tasks::none())
            }
            Message::PasswordInput(_) => Ok(Tasks::single(async move {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                Ok(Message::Login)
            })
            .with_label("Password Input")
            .on_failure(|report| Message::ProcessPasswordFailed(format!("{:?}", report)))),
            Message::Login => {
                self.logged_in = true;

                Ok(Tasks::none())
            }
            Message::ProcessPasswordFailed(reason) => Err(Report::msg(reason)),
        }
    }

    pub fn view(&self) -> View<Message> {
        View::new(
            Anchor::TOP,
            (1920u32, 20u32),
            container()
                .enabled(self.connection_panel)
                .with_child(
                    square()
                        .with_rect(Rect {
                            x: 0u32,
                            y: 0u32,
                            w: 1920u32,
                            h: 20u32,
                        })
                        .with_color(Color::BLUE)
                        .with_child(
                            text()
                                .with_string(self.logged_in.to_string())
                                .with_color(Color::RED)
                                .into(),
                        )
                        .into(),
                )
                .with_child(
                    button()
                        .with_rect(Rect {
                            x: 0u32,
                            y: 0u32,
                            w: 20u32,
                            h: 20u32,
                        })
                        .with_color(Color::GREEN)
                        .on_click(|_| Message::PasswordInput("password".to_string()))
                        .into(),
                )
                .into(),
        )
    }
}
