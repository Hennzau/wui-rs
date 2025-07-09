use std::time::Duration;

use wui_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    Application::new(MyApp::default(), MyApp::update, MyApp::views)?
        .run()
        .await
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
enum Message {
    Clicked,
    Escaped,
    Stop,
}

impl MyApp {
    fn update(&mut self, message: Message) -> Option<Task<Message>> {
        match message {
            Message::Clicked => {
                println!("Button clicked!");
                None
            }
            Message::Escaped => Some(Task::user(async move {
                println!("Stopping application...");

                tokio::time::sleep(Duration::from_millis(1000)).await;

                println!("Application will now exit.");

                Ok(Some(Message::Stop))
            })),
            Message::Stop => Some(Task::exit()),
        }
    }

    fn views(&self) -> ViewsBuilder {
        vec![
            view()
                .with_configuration(ViewConfiguration {
                    anchor: Anchor::TOP,
                    namespace: String::from("Top bar"),
                    ..Default::default()
                })
                .with_child(rect()),
            view()
                .with_configuration(ViewConfiguration {
                    anchor: Anchor::BOTTOM,
                    namespace: String::from("Bottom bar"),
                    ..Default::default()
                })
                .with_child(rect()),
        ]
    }
}
