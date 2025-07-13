use wui_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    Application::new(State::default(), State::update, State::views)?
        .run()
        .await
}

pub enum Message {
    Click,
    Hover,
}

struct State {}

impl Default for State {
    fn default() -> Self {
        Self {}
    }
}

impl State {
    fn update(&mut self, _: Message) {}

    fn views(&self) -> Views<Message> {
        vec![
            view()
                .label("bar.top")
                .size(1920, 24)
                .exclusive_zone(24)
                .child(rect().size(24, 24).child(rect().size(12, 12))),
            view()
                .label("bar.bottom")
                .size(1920, 24)
                .anchor(Anchor::BOTTOM)
                .exclusive_zone(24)
                .child(rect().size(24, 24).child(rect().size(12, 12))),
        ]
        .into()
    }
}
