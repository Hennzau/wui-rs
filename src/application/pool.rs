use std::{ops::Deref, sync::Arc};

use crate::*;

mod task;
pub use task::*;

pub struct Pool<Message> {
    pub(crate) tx: Sender<RunnableTask<Message>>,
    pub(crate) msg: Receiver<Message>,
}

impl<Message: 'static + Send + Sync> Pool<Message> {
    pub(crate) fn bsubmit(&self, task: RunnableTask<Message>) {
        self.tx.bsend(task);
    }

    pub(crate) async fn submit(&self, task: RunnableTask<Message>) {
        self.tx.send(task).await;
    }

    pub(crate) fn new(
        on_error: Option<impl Fn(Report) -> Message + 'static + Send + Sync>,
    ) -> Self {
        let on_error = Arc::new(on_error);

        let (tx, mut rx) = channel(128);
        let (tmsg, msg) = channel(256);

        let ttx: Sender<RunnableTask<Message>> = tx.clone();
        tokio::spawn(async move {
            while let Ok(task) = rx.recv().await {
                let signal = task.signal;

                match task.kind {
                    RunnableTaskKind::Future(fut) => {
                        let on_error = on_error.clone();

                        let msg = tmsg.clone();
                        tokio::spawn(async move {
                            let result = fut.await;
                            signal.map(|s| s.send(()));

                            match on_error.deref() {
                                Some(on_error) => {
                                    msg.send(result.unwrap_or_else(on_error)).await;
                                }
                                None => match result {
                                    Ok(message) => {
                                        msg.send(message).await;
                                    }
                                    Err(e) => {
                                        tracing::error!("Error {}", e);
                                    }
                                },
                            };
                        });
                    }
                    RunnableTaskKind::Batch(tasks) => {
                        let tx = ttx.clone();

                        tokio::spawn(async move {
                            let mut releases = Vec::new();

                            for mut t in tasks {
                                let (tsignal, release) = one_shot();

                                t.signal = Some(tsignal);

                                tx.send(t).await;

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
                    RunnableTaskKind::Then(mut first, mut second) => {
                        let tx = ttx.clone();
                        tokio::spawn(async move {
                            let (fsignal, release) = one_shot();
                            first.signal = Some(fsignal);

                            tx.send(*first).await;

                            release.await.unwrap_or_else(|e| {
                                tracing::error!("Failed to release first task: {}", e);
                            });

                            let (ssignal, release) = one_shot();
                            second.signal = Some(ssignal);
                            tx.send(*second).await;

                            release.await.unwrap_or_else(|e| {
                                tracing::error!("Failed to release second task: {}", e);
                            });

                            signal.map(|s| s.send(()));
                        });
                    }
                }
            }
        });

        Self { tx, msg }
    }

    pub(crate) fn pop(&mut self) -> Result<Message> {
        self.msg.try_recv()
    }
}
