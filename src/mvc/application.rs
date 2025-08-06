use crate::prelude::*;

pub(crate) enum ApplicationBehavior<Message> {
    Stop,
    Reset,

    Spawn(Element<Message>),
    Destroy(Label),
}

impl<A: 'static> ApplicationBehavior<A> {
    pub(crate) fn map<B: 'static + Send + Sync>(self, map: Map<A, B>) -> ApplicationBehavior<B> {
        match self {
            ApplicationBehavior::Stop => ApplicationBehavior::Stop,
            ApplicationBehavior::Reset => ApplicationBehavior::Reset,
            ApplicationBehavior::Spawn(element) => ApplicationBehavior::Spawn(element.map(map)),
            ApplicationBehavior::Destroy(label) => ApplicationBehavior::Destroy(label),
        }
    }
}

pub trait View<Message> {
    fn view(&self) -> Element<Message>;
}

pub trait Controller<Message> {
    fn controller(&mut self, message: Message) -> Task<Message>;
}

pub trait Application<Message> {
    fn run() -> impl Future<Output = Result<()>>;

    fn run_with_err(
        on_err: impl Fn(Report) -> Message + 'static + Send + Sync,
    ) -> impl Future<Output = Result<()>>;

    fn run_with_task(task: Task<Message>) -> impl Future<Output = Result<()>>;

    fn run_with_task_and_err(
        task: Task<Message>,
        on_err: impl Fn(Report) -> Message + 'static + Send + Sync,
    ) -> impl Future<Output = Result<()>>;
}

impl<T, Message: 'static + Send + Sync> Application<Message> for T
where
    T: 'static + View<Message> + Controller<Message> + Default + Send + Sync,
{
    async fn run() -> Result<()> {
        T::default()
            .run_app(None, None::<fn(Report) -> Message>)
            .await
    }

    async fn run_with_err(
        on_err: impl Fn(Report) -> Message + 'static + Send + Sync,
    ) -> Result<()> {
        T::default().run_app(None, Some(on_err)).await
    }

    async fn run_with_task(task: Task<Message>) -> Result<()> {
        T::default()
            .run_app(Some(task), None::<fn(Report) -> Message>)
            .await
    }

    async fn run_with_task_and_err(
        task: Task<Message>,
        on_err: impl Fn(Report) -> Message + 'static + Send + Sync,
    ) -> Result<()> {
        T::default().run_app(Some(task), Some(on_err)).await
    }
}

trait Runnable<Message> {
    fn run_app(
        self,

        task: Option<Task<Message>>,
        on_err: Option<impl Fn(Report) -> Message + 'static + Send + Sync>,
    ) -> impl Future<Output = Result<()>>;
}

impl<T, Message: 'static + Send + Sync> Runnable<Message> for T
where
    T: 'static + View<Message> + Controller<Message> + Default + Send + Sync,
{
    async fn run_app(
        self,

        task: Option<Task<Message>>,
        on_err: Option<impl Fn(Report) -> Message + 'static + Send + Sync>,
    ) -> Result<()> {
        let (msg, mut rmsg) = channel::<Message>();
        let (behavior, mut rbehavior) = channel::<ApplicationBehavior<Message>>();

        let (pool, tasks) = {
            let pool = TaskPool::new(behavior.clone(), msg.clone());
            let tasks = pool.tx();

            if let Some(task) = task {
                tasks.send(task);
            }

            let pool = tokio::spawn(pool.run(on_err));

            (pool, tasks)
        };

        let (backend, elements) = {
            let backend = Backend::new(msg.clone()).await?;
            let elements = backend.tx();

            let backend = tokio::spawn(backend.run());

            (backend, elements)
        };

        let app = tokio::spawn(async move {
            let mut model = self;

            let element = model.view();
            let mut labels = element.labels();
            elements.send(Request::Spawn(element));

            loop {
                tokio::select! {
                    Ok(message) = rmsg.recv() => {
                        let task = model.controller(message);

                        tasks.send(task);

                        let element = model.view();
                        let new_labels = element.labels();

                        elements.send(Request::Spawn(element));

                        for label in labels {
                            if !new_labels.contains(&label) {
                                if let Some(label) = label {
                                    elements.send(Request::Destroy(label));
                                }
                            }
                        }

                        labels = new_labels;
                    }
                    Ok(behavior) = rbehavior.recv() => {
                        match behavior {
                            ApplicationBehavior::Stop => break,
                            ApplicationBehavior::Reset => {
                                model = T::default();
                            },
                            ApplicationBehavior::Spawn(element) => {
                                elements.send(Request::Spawn(element));
                            },
                            ApplicationBehavior::Destroy(label) => {
                                elements.send(Request::Destroy(label));
                            }
                        }
                    }
                }
            }

            Ok::<(), Report>(())
        });

        let ctrl_c = tokio::signal::ctrl_c();

        tokio::select! {
            result = app => {
                tracing::info!("Application finished with result: {:?}", result);

                result.map_err(Report::msg)?
            }
            result = pool => {
                tracing::info!("Task pool finished with result: {:?}", result);

                result.map_err(Report::msg)
            }
            result = backend => {
                tracing::info!("Backend finished with result: {:?}", result);

                result.map_err(Report::msg)?
            }
            result = ctrl_c => {
                tracing::info!("Received Ctrl+C, stopping application");

                result.map_err(Report::msg)
            }
        }
    }
}
