use wui_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    Application::new(Model::default, Model::controller, Model::view)
        .run(|e| {
            tracing::error!("Error in application: {:?}", e);

            Message::Stop
        })
        .await
}

enum Message {
    Stop,
}

#[derive(Default)]
struct Model;

impl Model {
    fn controller(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Stop => Task::stop(),
        }
    }

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
