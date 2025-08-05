use std::sync::Arc;

use crate::prelude::*;

pub(crate) struct TaskPool<Message> {
    tx: Sender<Task<Message>>,
    rx: Receiver<Task<Message>>,

    app: Sender<ApplicationBehavior<Message>>,
    msg: Sender<Message>,
}

impl<Message: 'static> TaskPool<Message> {
    pub(crate) fn new(app: Sender<ApplicationBehavior<Message>>, msg: Sender<Message>) -> Self {
        let (tx, rx) = channel();

        TaskPool { tx, rx, app, msg }
    }

    pub(crate) fn tx(&self) -> Sender<Task<Message>> {
        self.tx.clone()
    }
}
impl<Message: 'static + Send + Sync> TaskPool<Message> {
    pub(crate) async fn run(
        mut self,
        on_error: impl Fn(Report) -> Message + 'static + Send + Sync,
    ) {
        tracing::info!("TaskPool started");

        let on_error = Arc::new(on_error);

        while let Ok(task) = self.rx.recv().await {
            let signal = task.signal;

            match task.inner {
                TaskInner::None => {
                    signal.map(|s| s.send(()));
                }
                TaskInner::Application(behavior) => {
                    self.app.send(behavior);

                    signal.map(|s| s.send(()));
                }
                TaskInner::Future(fut) => {
                    let msg = self.msg.clone();
                    let on_error = on_error.clone();
                    tokio::spawn(async move {
                        let result = fut.await;
                        signal.map(|s| s.send(()));

                        msg.send(result.unwrap_or_else(|e| on_error(e)))
                    });
                }
                TaskInner::Batch(tasks) => {
                    let tx = self.tx.clone();

                    tokio::spawn(async move {
                        let mut releases = Vec::new();

                        for mut t in tasks {
                            let (tsignal, release) = one_shot();

                            t.signal = Some(tsignal);

                            tx.send(t);

                            releases.push(release);
                        }

                        for release in releases {
                            release.await.unwrap_or_else(|e| {
                                tracing::error!("Failed to release task: {}", e);
                            });
                        }

                        signal.map(|s| s.send(()));
                    });
                }
                TaskInner::Then(mut first, mut second) => {
                    let tx = self.tx.clone();
                    tokio::spawn(async move {
                        let (fsignal, release) = one_shot();
                        first.signal = Some(fsignal);

                        tx.send(*first);

                        release.await.unwrap_or_else(|e| {
                            tracing::error!("Failed to release first task: {}", e);
                        });

                        let (ssignal, release) = one_shot();
                        second.signal = Some(ssignal);
                        tx.send(*second);

                        release.await.unwrap_or_else(|e| {
                            tracing::error!("Failed to release second task: {}", e);
                        });

                        signal.map(|s| s.send(()));
                    });
                }
            }
        }
    }
}
