use eyre::Result;
use wui_rs::prelude::*;

fn main() -> Result<()> {
    application(Bar::default, Bar::update, vec![Bar::view])
        .with_init_message(Message::Status(0))
        .run(Message::TaskFailed)
}

struct Bar {}

#[derive(Debug, Clone)]
enum Message {
    Status(u32),
    TaskFailed(String),
}

impl Bar {
    pub fn update(&mut self, message: Message) -> Result<Task<Message>> {
        match message {
            Message::Status(status) => {
                println!("Status: {}", status);
                Ok(Task::none())
            }
            Message::TaskFailed(report) => Err(eyre::eyre!(report)),
        }
    }
    pub fn view(&self) -> Element<Message> {
        Element::none()
    }
}

impl Default for Bar {
    fn default() -> Self {
        Bar {}
    }
}
