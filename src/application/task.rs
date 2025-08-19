use crate::RunnableTask;

pub(crate) enum TaskKind<Message> {
    None,
    Stop,
    Reset,

    Runnable(RunnableTask<Message>),
}

pub struct Task<Message> {
    pub(crate) kind: TaskKind<Message>,
}

impl<Message> Task<Message> {
    pub fn none() -> Self {
        Self {
            kind: TaskKind::None,
        }
    }

    pub fn stop() -> Self {
        Self {
            kind: TaskKind::Stop,
        }
    }

    pub fn reset() -> Self {
        Self {
            kind: TaskKind::Reset,
        }
    }

    pub fn runnable(task: RunnableTask<Message>) -> Self {
        Self {
            kind: TaskKind::Runnable(task),
        }
    }
}
