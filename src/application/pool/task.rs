use std::pin::Pin;

use crate::*;

pub(crate) type OneShot = tokio::sync::oneshot::Sender<()>;
pub(crate) fn one_shot() -> (OneShot, tokio::sync::oneshot::Receiver<()>) {
    tokio::sync::oneshot::channel()
}

pub(crate) enum TaskKind<Message> {
    Future(Pin<Box<dyn Future<Output = Result<Message>> + Send + Sync + 'static>>),

    Batch(Vec<Task<Message>>),
    Then(Box<Task<Message>>, Box<Task<Message>>),
}

pub struct Task<Message> {
    pub(crate) kind: TaskKind<Message>,
    pub(crate) signal: Option<OneShot>,
}

impl<Message> Task<Message> {
    pub fn future(future: impl Future<Output = Result<Message>> + Send + Sync + 'static) -> Self {
        Self {
            kind: TaskKind::Future(Box::pin(future)),
            signal: None,
        }
    }

    pub fn batch(self, other: Task<Message>) -> Self {
        Self {
            kind: TaskKind::Batch(vec![self, other]),
            signal: None,
        }
    }

    pub fn then(self, other: Task<Message>) -> Self {
        Self {
            kind: TaskKind::Then(Box::new(self), Box::new(other)),
            signal: None,
        }
    }
}

impl<Message: 'static + Send + Sync> Task<Message> {
    pub fn msg(message: Message) -> Self {
        Self {
            kind: TaskKind::Future(Box::pin(async move { Ok(message) })),
            signal: None,
        }
    }
}
