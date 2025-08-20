use std::{ops::Deref, sync::Arc};
use tokio::sync::mpsc::{Receiver, Sender, channel};
use winit::event_loop::EventLoopProxy;

use crate::*;

mod task;
pub use task::*;

pub(crate) struct Pool<Message> {
    pub(crate) tx: Sender<Task<Message>>,
    pub(crate) msg: Receiver<Message>,
}

impl<Message: 'static + Send + Sync> Pool<Message> {
    pub(crate) fn bsubmit(&self, task: Task<Message>) {
        if let Err(e) = self.tx.blocking_send(task) {
            tracing::error!("Failed to submit task: {}", e);
        }
    }

    async fn send<T>(sender: &Sender<T>, value: T) {
        if let Err(e) = sender.send(value).await {
            tracing::error!("Failed to send value: {}", e);
        }
    }

    pub(crate) fn new(
        on_error: Option<impl Fn(Report) -> Message + 'static + Send + Sync>,
        proxy: EventLoopProxy,
    ) -> Self {
        let on_error = Arc::new(on_error);

        let (tx, mut rx) = channel(128);
        let (tmsg, msg) = channel(256);

        let ttx: Sender<Task<Message>> = tx.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime");

            rt.block_on(async move {
                while let Some(task) = rx.recv().await {
                    let signal = task.signal;

                    match task.kind {
                        TaskKind::Future(fut) => {
                            let on_error = on_error.clone();

                            let msg = tmsg.clone();
                            let proxy = proxy.clone();
                            tokio::spawn(async move {
                                let result = fut.await;
                                signal.map(|s| s.send(()));

                                match on_error.deref() {
                                    Some(on_error) => {
                                        Self::send(&msg, result.unwrap_or_else(on_error)).await;
                                    }
                                    None => match result {
                                        Ok(message) => {
                                            Self::send(&msg, message).await;
                                        }
                                        Err(e) => {
                                            tracing::error!("Error {}", e);
                                        }
                                    },
                                };

                                proxy.wake_up();
                            });
                        }
                        TaskKind::Batch(tasks) => {
                            let tx = ttx.clone();

                            tokio::spawn(async move {
                                let mut releases = Vec::new();

                                for mut t in tasks {
                                    let (tsignal, release) = one_shot();

                                    t.signal = Some(tsignal);

                                    Self::send(&tx, t).await;

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
                        TaskKind::Then(mut first, mut second) => {
                            let tx = ttx.clone();
                            tokio::spawn(async move {
                                let (fsignal, release) = one_shot();
                                first.signal = Some(fsignal);

                                Self::send(&tx, *first).await;

                                release.await.unwrap_or_else(|e| {
                                    tracing::error!("Failed to release first task: {}", e);
                                });

                                let (ssignal, release) = one_shot();
                                second.signal = Some(ssignal);
                                Self::send(&tx, *second).await;

                                release.await.unwrap_or_else(|e| {
                                    tracing::error!("Failed to release second task: {}", e);
                                });

                                signal.map(|s| s.send(()));
                            });
                        }
                    }
                }

                Ok::<(), Report>(())
            })
        });

        Self { tx, msg }
    }

    pub(crate) fn try_recv(&mut self) -> Result<Message> {
        self.msg.try_recv().map_err(Report::msg)
    }
}
