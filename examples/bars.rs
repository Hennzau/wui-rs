use std::time::Duration;

use wui_rs::*;

#[tokio::main]
async fn main() -> Result<()> {
    use app::TaskedApplication;

    Model::default()
        .run(RunnableTask::future(async move {
            tokio::time::sleep(Duration::from_millis(1000)).await;

            Ok(Message::Hello)
        }))
        .await
}

#[derive(Debug)]
enum Message {
    Hello,
}

#[derive(Default)]
struct Model {}

impl Controller<Message> for Model {
    fn controller(&mut self, msg: Message) -> Task<Message> {
        println!("Received message: {:?}", msg);

        Task::runnable(RunnableTask::future(async move {
            tokio::time::sleep(Duration::from_millis(1000)).await;

            Ok(Message::Hello)
        }))
    }
}

impl View<Message> for Model {
    fn view(&self) -> impl IntoRootWidgets<Message> {
        vec![
            root("bar.top")
                .width(2133)
                .height(24)
                .background(palette::css::RED)
                .anchor(Anchor::Top(24)),
            root("bar.bottom")
                .width(2133)
                .height(24)
                .background(palette::css::BLUE)
                .anchor(Anchor::Bottom(24)),
        ]
    }
}
