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

pub trait IntoTask<Message> {
    fn into_task(self) -> Task<Message>;
}

impl<Message> IntoTask<Message> for Task<Message> {
    fn into_task(self) -> Task<Message> {
        self
    }
}

impl<Message> IntoTask<Message> for RunnableTask<Message> {
    fn into_task(self) -> Task<Message> {
        Task::runnable(self)
    }
}
