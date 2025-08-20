use crate::Task;

pub(crate) enum CommandKind<Message> {
    None,
    Stop,
    Reset,

    Task(Task<Message>),
}

pub struct Command<Message> {
    pub(crate) kind: CommandKind<Message>,
}

impl<Message> Command<Message> {
    pub fn none() -> Self {
        Self {
            kind: CommandKind::None,
        }
    }

    pub fn stop() -> Self {
        Self {
            kind: CommandKind::Stop,
        }
    }

    pub fn reset() -> Self {
        Self {
            kind: CommandKind::Reset,
        }
    }

    pub fn task(task: Task<Message>) -> Self {
        Self {
            kind: CommandKind::Task(task),
        }
    }
}

pub trait IntoCommand<Message> {
    fn into_command(self) -> Command<Message>;
}

impl<Message> IntoCommand<Message> for Command<Message> {
    fn into_command(self) -> Command<Message> {
        self
    }
}

impl<Message> IntoCommand<Message> for Task<Message> {
    fn into_command(self) -> Command<Message> {
        Command::task(self)
    }
}
