use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::*;

mod task;
pub use task::*;

mod pool;
pub use pool::*;

pub(crate) struct Application<Model, Message> {
    pub(crate) model: Model,

    pub(crate) widgets: Widgets<Message>,

    pub(crate) msg: Vec<Message>,

    pub(crate) pool: Pool<Message>,
}

impl<Model, Message: 'static + Send + Sync> Application<Model, Message>
where
    Model: 'static + Default + Send + Sync + View<Message> + Controller<Message>,
{
    pub(crate) fn rebuild_view(&mut self, event_loop: &dyn ActiveEventLoop) {
        if let Err(e) = self
            .widgets
            .reconciliate(self.model.view().into_root_widgets(), event_loop)
        {
            eprintln!("Error during widget reconciliation: {}", e);
            event_loop.exit();
        }
    }

    pub(crate) fn handle_task(&mut self, task: Task<Message>, event_loop: &dyn ActiveEventLoop) {
        match task.kind {
            TaskKind::None => {}
            TaskKind::Reset => {
                self.model = Model::default();
            }
            TaskKind::Stop => {
                event_loop.exit();
            }
            TaskKind::Runnable(task) => {
                self.pool.bsubmit(task);
            }
        }
    }
}

impl<Model, Message: 'static + Send + Sync> ApplicationHandler for Application<Model, Message>
where
    Model: 'static + Default + Send + Sync + View<Message> + Controller<Message>,
{
    fn proxy_wake_up(&mut self, event_loop: &dyn ActiveEventLoop) {
        let mut updated = false;

        while let Ok(message) = self.pool.try_recv() {
            updated = true;

            let task = self.model.controller(message).into_task();
            self.handle_task(task, event_loop);
        }

        if updated {
            self.rebuild_view(event_loop);
        }

        self.widgets.request_redraw();
    }

    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        self.rebuild_view(event_loop);
        self.widgets.request_redraw();
    }

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if event == WindowEvent::RedrawRequested {
            if let Err(e) = self.widgets.redraw(id) {
                println!("Error during redraw: {}", e);
                event_loop.exit();
                return;
            }

            return;
        }

        if let Err(e) = self.widgets.handle_event(&mut self.msg, id, event) {
            println!("Error handling event: {}", e);
            event_loop.exit();
            return;
        }

        if self.widgets.widgets.is_empty() {
            event_loop.exit();

            return;
        }

        let mut updated = false;

        while let Some(message) = self.msg.pop() {
            updated = true;

            let task = self.model.controller(message).into_task();
            self.handle_task(task, event_loop);
        }

        if updated {
            self.rebuild_view(event_loop);
        }

        self.widgets.request_redraw();
    }
}

pub trait View<Message> {
    fn view(&self) -> impl IntoRootWidgets<Message>;
}

pub trait Controller<Message> {
    fn controller(&mut self, message: Message) -> impl IntoTask<Message>;
}

pub mod app {
    use winit::event_loop::EventLoop;

    use super::*;

    pub trait BasicApplication<Message> {
        fn run() -> Result<()>;
    }

    impl<Model, Message: 'static + Send + Sync> BasicApplication<Message> for Model
    where
        Model: 'static + Default + Send + Sync + View<Message> + Controller<Message>,
    {
        fn run() -> Result<()> {
            let event_loop = EventLoop::new()?;

            let model = Model::default();
            let widgets = Widgets::new();
            let msg = Vec::new();
            let pool = Pool::new(None::<fn(Report) -> Message>, event_loop.create_proxy());

            event_loop
                .run_app(Application {
                    msg,
                    model,
                    widgets,
                    pool,
                })
                .map_err(Report::msg)
        }
    }

    pub trait ErrorHandledBasicApplication<Message> {
        fn run(on_error: impl Fn(Report) -> Message + 'static + Send + Sync) -> Result<()>;
    }

    impl<Model, Message: 'static + Send + Sync> ErrorHandledBasicApplication<Message> for Model
    where
        Model: 'static + Default + Send + Sync + View<Message> + Controller<Message>,
    {
        fn run(on_error: impl Fn(Report) -> Message + 'static + Send + Sync) -> Result<()> {
            let event_loop = EventLoop::new()?;

            let model = Model::default();
            let widgets = Widgets::new();
            let msg = Vec::new();

            let pool = Pool::new(Some(on_error), event_loop.create_proxy());

            event_loop
                .run_app(Application {
                    msg,
                    model,
                    widgets,
                    pool,
                })
                .map_err(Report::msg)
        }
    }

    pub trait TaskedApplication<Message> {
        fn run(task: RunnableTask<Message>) -> Result<()>;
    }

    impl<Model, Message: 'static + Send + Sync> TaskedApplication<Message> for Model
    where
        Model: 'static + Default + Send + Sync + View<Message> + Controller<Message>,
    {
        fn run(task: RunnableTask<Message>) -> Result<()> {
            let event_loop = EventLoop::new()?;

            let model = Model::default();
            let widgets = Widgets::new();
            let msg = Vec::new();

            let pool = Pool::new(None::<fn(Report) -> Message>, event_loop.create_proxy());
            pool.bsubmit(task);

            event_loop
                .run_app(Application {
                    msg,
                    model,
                    widgets,
                    pool,
                })
                .map_err(Report::msg)
        }
    }

    pub trait ErrorHandledTaskedApplication<Message> {
        fn run(
            on_error: impl Fn(Report) -> Message + 'static + Send + Sync,
            task: RunnableTask<Message>,
        ) -> Result<()>;
    }

    impl<Model, Message: 'static + Send + Sync> ErrorHandledTaskedApplication<Message> for Model
    where
        Model: 'static + Default + Send + Sync + View<Message> + Controller<Message>,
    {
        fn run(
            on_error: impl Fn(Report) -> Message + 'static + Send + Sync,
            task: RunnableTask<Message>,
        ) -> Result<()> {
            let event_loop = EventLoop::new()?;

            let model = Model::default();
            let widgets = Widgets::new();
            let msg = Vec::new();

            let pool = Pool::new(Some(on_error), event_loop.create_proxy());
            pool.bsubmit(task);

            event_loop
                .run_app(Application {
                    msg,
                    model,
                    widgets,
                    pool,
                })
                .map_err(Report::msg)
        }
    }
}
