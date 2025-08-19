use std::pin::Pin;

use crate::*;

pub(crate) type OneShot = tokio::sync::oneshot::Sender<()>;
pub(crate) fn one_shot() -> (OneShot, tokio::sync::oneshot::Receiver<()>) {
    tokio::sync::oneshot::channel()
}

pub(crate) enum RunnableTaskKind<Message> {
    Future(Pin<Box<dyn Future<Output = Result<Message>> + Send + Sync + 'static>>),

    Batch(Vec<RunnableTask<Message>>),
    Then(Box<RunnableTask<Message>>, Box<RunnableTask<Message>>),
}

pub struct RunnableTask<Message> {
    pub(crate) kind: RunnableTaskKind<Message>,
    pub(crate) signal: Option<OneShot>,
}

impl<Message> RunnableTask<Message> {
    pub fn future(future: impl Future<Output = Result<Message>> + Send + Sync + 'static) -> Self {
        Self {
            kind: RunnableTaskKind::Future(Box::pin(future)),
            signal: None,
        }
    }

    pub fn batch(self, other: RunnableTask<Message>) -> Self {
        Self {
            kind: RunnableTaskKind::Batch(vec![self, other]),
            signal: None,
        }
    }

    pub fn then(self, other: RunnableTask<Message>) -> Self {
        Self {
            kind: RunnableTaskKind::Then(Box::new(self), Box::new(other)),
            signal: None,
        }
    }
}

impl<Message: 'static + Send + Sync> RunnableTask<Message> {
    pub fn msg(message: Message) -> Self {
        Self {
            kind: RunnableTaskKind::Future(Box::pin(async move { Ok(message) })),
            signal: None,
        }
    }
}
