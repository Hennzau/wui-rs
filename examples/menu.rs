use std::time::Duration;

use wui_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    Model::run_with_task_and_err(Task::msg(Message::Prepare), Message::Error).await
}

enum Message {
    Error(Report),
    Stop,
    Prepare,
    Change,
}

#[derive(Default)]
struct Model {
    changed: bool,
}

impl Controller<Message> for Model {
    fn controller(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Error(report) => {
                tracing::error!("Error: {}", report);

                Task::msg(Message::Stop)
            }
            Message::Stop => Task::stop(),
            Message::Prepare => Task::future(async move {
                tokio::time::sleep(Duration::from_secs(1)).await;

                Ok(Message::Change)
            }),
            Message::Change => {
                self.changed = true;

                Task::future(async move {
                    tokio::time::sleep(Duration::from_secs(1)).await;

                    Ok(Message::Stop)
                })
            }
        }
    }
}
impl View<Message> for Model {
    fn view(&self) -> Element<Message> {
        let elements = container().with(
            empty()
                .label("menu.center")
                .size(Size {
                    width: match self.changed {
                        true => 1920,
                        false => 1280,
                    },
                    height: match self.changed {
                        true => 1080,
                        false => 720,
                    },
                })
                .display_mode(DisplayMode::Layered {
                    location: Location {
                        x: match self.changed {
                            true => 0,
                            false => (1920 - 1280) / 2,
                        },
                        y: match self.changed {
                            true => 0,
                            false => (1080 - 720) / 2,
                        },
                        ..Default::default()
                    },
                    kind: LayerKind::default(),
                }),
        );

        elements.element()
    }
}
