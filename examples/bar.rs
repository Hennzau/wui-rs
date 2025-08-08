use wui_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    Model::run_with_err(Message::Error).await
}

enum Message {
    Error(Report),
    Stop,
}

#[derive(Default)]
struct Model;

impl Controller<Message> for Model {
    fn controller(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Stop => Task::stop(),
            Message::Error(report) => {
                tracing::error!("Error: {}", report);

                Task::msg(Message::Stop)
            }
        }
    }
}

impl View<Message> for Model {
    fn view(&self) -> Element<Message> {
        let elements = container()
            .with(
                empty()
                    .label("bar.top")
                    .size(Size {
                        width: 1920,
                        height: 24,
                    })
                    .display_mode(DisplayMode::Layered {
                        location: Location {
                            x: 0,
                            y: 0,
                            side: Some(Side::Top),
                            exclusive: 24,
                            ..Default::default()
                        },
                        kind: LayerKind::default(),
                    }),
            )
            .with(
                empty()
                    .label("bar.bottom")
                    .size(Size {
                        width: 1920,
                        height: 24,
                    })
                    .display_mode(DisplayMode::Layered {
                        location: Location {
                            x: 0,
                            y: 0,
                            side: Some(Side::Bottom),
                            exclusive: 24,
                            ..Default::default()
                        },
                        kind: LayerKind::default(),
                    }),
            );

        elements.element()
    }
}
