use std::pin::Pin;

use crate::prelude::*;

pub(crate) type OneShot = tokio::sync::oneshot::Sender<()>;
pub(crate) fn one_shot() -> (OneShot, tokio::sync::oneshot::Receiver<()>) {
    tokio::sync::oneshot::channel()
}

pub(crate) enum TaskInner<Message> {
    Future(Pin<Box<dyn Future<Output = Result<Message>> + Send + Sync + 'static>>),

    Batch(Vec<Task<Message>>),
    Then(Box<Task<Message>>, Box<Task<Message>>),

    None,

    Application(ApplicationBehavior<Message>),
}

pub struct Task<Message> {
    pub(crate) inner: TaskInner<Message>,
    pub(crate) signal: Option<OneShot>,
}

impl<Message> Task<Message> {
    pub fn future(future: impl Future<Output = Result<Message>> + Send + Sync + 'static) -> Self {
        Self {
            inner: TaskInner::Future(Box::pin(future)),
            signal: None,
        }
    }

    pub fn batch(self, other: Task<Message>) -> Self {
        Self {
            inner: TaskInner::Batch(vec![self, other]),
            signal: None,
        }
    }

    pub fn then(self, other: Task<Message>) -> Self {
        Self {
            inner: TaskInner::Then(Box::new(self), Box::new(other)),
            signal: None,
        }
    }

    pub fn none() -> Self {
        Self {
            inner: TaskInner::None,
            signal: None,
        }
    }

    pub fn stop() -> Self {
        Self {
            inner: TaskInner::Application(ApplicationBehavior::Stop),
            signal: None,
        }
    }

    pub fn reset() -> Self {
        Self {
            inner: TaskInner::Application(ApplicationBehavior::Reset),
            signal: None,
        }
    }

    pub fn spawn(self, element: impl IntoElement<Message>) -> Self {
        Self {
            inner: TaskInner::Application(ApplicationBehavior::Spawn(element.element())),
            signal: None,
        }
    }

    pub fn destroy(self, label: Label) -> Self {
        Self {
            inner: TaskInner::Application(ApplicationBehavior::Destroy(label)),
            signal: None,
        }
    }
}

impl<Message: 'static + Send + Sync> Task<Message> {
    pub fn msg(message: Message) -> Self {
        Self {
            inner: TaskInner::Future(Box::pin(async move { Ok(message) })),
            signal: None,
        }
    }

    pub fn map<NewMessage: 'static + Send + Sync>(
        self,
        map: Map<Message, NewMessage>,
    ) -> Task<NewMessage> {
        Task {
            inner: match self.inner {
                TaskInner::Future(fut) => TaskInner::Future(Box::pin(async move {
                    let message = fut.await?;

                    Ok(map.map(message))
                })),
                TaskInner::Batch(tasks) => TaskInner::Batch(
                    tasks
                        .into_iter()
                        .map(|task| task.map(map.clone()))
                        .collect(),
                ),
                TaskInner::Then(first, second) => TaskInner::Then(
                    Box::new(first.map(map.clone())),
                    Box::new(second.map(map.clone())),
                ),
                TaskInner::None => TaskInner::None,
                TaskInner::Application(app_behavior) => {
                    TaskInner::Application(app_behavior.map(map))
                }
            },
            signal: self.signal,
        }
    }
}
