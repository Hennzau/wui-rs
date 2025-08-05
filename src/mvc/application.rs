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

pub(crate) type ModelFn<Model> = Box<dyn Fn() -> Model + Send>;
pub(crate) type ControllerFn<Model, Message> =
    Box<dyn Fn(&mut Model, Message) -> Task<Message> + Send>;
pub(crate) type ViewFn<Model, Message> = Box<dyn Fn(&Model) -> Element<Message> + Send>;

pub struct Application<Model, Message> {
    pub(crate) model: ModelFn<Model>,
    pub(crate) controller: ControllerFn<Model, Message>,
    pub(crate) view: ViewFn<Model, Message>,

    pub(crate) task: Option<Task<Message>>,
}

impl<Model, Message> Application<Model, Message> {
    pub fn new(
        model: impl Fn() -> Model + Send + 'static,
        controller: impl Fn(&mut Model, Message) -> Task<Message> + Send + 'static,
        view: impl Fn(&Model) -> Element<Message> + Send + 'static,
    ) -> Self {
        Self {
            model: Box::new(model),
            controller: Box::new(controller),
            view: Box::new(view),
            task: None,
        }
    }

    pub fn task(self, task: Task<Message>) -> Self {
        Self {
            task: Some(task),
            ..self
        }
    }
}

impl<Model: 'static + Send + Sync, Message: 'static + Send + Sync> Application<Model, Message> {
    pub(crate) async fn jobs(
        self,
        on_error: impl Fn(Report) -> Message + 'static + Send + Sync,
    ) -> Result<(
        JoinHandle<Result<()>>,
        JoinHandle<()>,
        JoinHandle<Result<()>>,
    )> {
        let (msg, mut rmsg) = channel::<Message>();
        let (behavior, mut rbehavior) = channel::<ApplicationBehavior<Message>>();

        let (pool, tasks) = {
            let pool = TaskPool::new(behavior.clone(), msg.clone());
            let tasks = pool.tx();

            if let Some(task) = self.task {
                tasks.send(task);
            }

            let pool = tokio::spawn(pool.run(on_error));

            (pool, tasks)
        };

        let (backend, elements) = {
            let backend = Backend::new(msg.clone()).await?;
            let elements = backend.tx();

            let backend = tokio::spawn(backend.run());

            (backend, elements)
        };

        let app = tokio::spawn(async move {
            let mut model = (self.model)();

            let element = (self.view)(&model);
            let mut labels = element.labels();
            elements.send(Request::Spawn(element));

            loop {
                tokio::select! {
                    Ok(message) = rmsg.recv() => {
                        let task = (self.controller)(&mut model, message);

                        tasks.send(task);

                        let element = (self.view)(&model);
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
                                model = (self.model)();
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

            Ok(())
        });

        Ok((app, pool, backend))
    }

    pub async fn run(
        self,
        on_error: impl Fn(Report) -> Message + 'static + Send + Sync,
    ) -> Result<()> {
        let (app, pool, backend) = self.jobs(on_error).await?;

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
