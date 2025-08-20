use crate::*;

mod pool;
pub use pool::*;

mod task;
pub use task::*;

pub trait View<Message> {
    fn view(&self) -> impl IntoRootWidgets<Message>;
}

pub trait Controller<Message> {
    fn controller(&mut self, message: Message) -> Task<Message>;
}

pub mod app {
    use super::*;

    pub trait BasicApplication<Message> {
        fn run(self) -> impl Future<Output = Result<()>>;
    }

    impl<T, Message: 'static + Send + Sync> BasicApplication<Message> for T
    where
        T: 'static + Controller<Message> + View<Message> + Default + Send + Sync,
    {
        async fn run(self) -> Result<()> {
            let wgpu = Wgpu::new();
            let (wl, mut event_queue) = Wl::new()?;

            let shell = Shell::<Message>::new(event_queue.handle(), wl, wgpu);
            let mut client = Client::new(shell, &event_queue.handle());

            let mut pool = Pool::<Message>::new(None::<fn(Report) -> Message>);

            let mut model = self;

            std::thread::spawn(move || {
                let elements = model.view().root_widgets();
                if let Err(e) = client.widgets.reconciliate(elements, &mut client.shell) {
                    println!("Error {}", e);
                }

                'main: loop {
                    match event_queue.blocking_dispatch(&mut client) {
                        Ok(len) => {
                            if len == 0 {
                                continue 'main;
                            }
                        }
                        Err(e) => {
                            println!("Error while dispatching events: {}", e);
                            break 'main;
                        }
                    }

                    let mut update = false;

                    'inner: loop {
                        let msg_opt = client
                            .msg
                            .pop()
                            .map(Ok::<Message, Report>)
                            .or_else(|| pool.pop().ok().map(Ok::<Message, Report>));

                        update = update || msg_opt.is_some();

                        match msg_opt {
                            Some(Ok(msg)) => match model.controller(msg).kind {
                                TaskKind::None => continue 'inner,
                                TaskKind::Stop => break 'main,
                                TaskKind::Reset => {
                                    model = T::default();
                                    update = true;
                                }
                                TaskKind::Runnable(task) => {
                                    pool.bsubmit(task);
                                }
                            },
                            _ => break 'inner,
                        }
                    }

                    if client.shell.surfaces() == 0 {
                        break 'main;
                    }

                    if update {
                        let elements = model.view().root_widgets();
                        if let Err(e) = client.widgets.reconciliate(elements, &mut client.shell) {
                            println!("Error {}", e);
                        }
                    }

                    if let Err(e) = client.widgets.draw(&mut client.shell) {
                        println!("Error {}", e);
                    }
                }
            });

            tokio::signal::ctrl_c().await?;

            Ok(())
        }
    }

    pub trait BasicApplicationWithErrorHandling<Message> {
        fn run(
            self,
            on_error: impl Fn(Report) -> Message + 'static + Send + Sync,
        ) -> impl Future<Output = Result<()>>;
    }

    impl<T, Message: 'static + Send + Sync> BasicApplicationWithErrorHandling<Message> for T
    where
        T: 'static + Controller<Message> + View<Message> + Default + Send + Sync,
    {
        async fn run(
            self,
            on_error: impl Fn(Report) -> Message + 'static + Send + Sync,
        ) -> Result<()> {
            let wgpu = Wgpu::new();
            let (wl, mut event_queue) = Wl::new()?;

            let shell = Shell::<Message>::new(event_queue.handle(), wl, wgpu);
            let mut client = Client::new(shell, &event_queue.handle());

            let mut pool = Pool::<Message>::new(Some(on_error));

            let mut model = self;

            std::thread::spawn(move || {
                let elements = model.view().root_widgets();
                if let Err(e) = client.widgets.reconciliate(elements, &mut client.shell) {
                    println!("Error {}", e);
                }

                'main: loop {
                    if let Err(e) = event_queue.blocking_dispatch(&mut client) {
                        println!("Error while dispatching events: {}", e);
                        break 'main;
                    }

                    let mut update = false;

                    'inner: loop {
                        let msg_opt = client
                            .msg
                            .pop()
                            .map(Ok::<Message, Report>)
                            .or_else(|| pool.pop().ok().map(Ok::<Message, Report>));

                        update = update || msg_opt.is_some();

                        match msg_opt {
                            Some(Ok(msg)) => match model.controller(msg).kind {
                                TaskKind::None => continue 'inner,
                                TaskKind::Stop => break 'main,
                                TaskKind::Reset => {
                                    model = T::default();
                                    update = true;
                                }
                                TaskKind::Runnable(task) => {
                                    pool.bsubmit(task);
                                }
                            },
                            _ => break 'inner,
                        }
                    }

                    if client.shell.surfaces() == 0 {
                        break 'main;
                    }

                    if update {
                        let elements = model.view().root_widgets();
                        if let Err(e) = client.widgets.reconciliate(elements, &mut client.shell) {
                            println!("Error {}", e);
                        }
                    }

                    if let Err(e) = client.widgets.draw(&mut client.shell) {
                        println!("Error {}", e);
                    }
                }
            });

            tokio::signal::ctrl_c().await?;

            Ok(())
        }
    }

    pub trait TaskedApplication<Message> {
        fn run(self, task: RunnableTask<Message>) -> impl Future<Output = Result<()>>;
    }

    impl<T, Message: 'static + Send + Sync> TaskedApplication<Message> for T
    where
        T: 'static + Controller<Message> + View<Message> + Default + Send + Sync,
    {
        async fn run(self, task: RunnableTask<Message>) -> Result<()> {
            let wgpu = Wgpu::new();
            let (wl, mut event_queue) = Wl::new()?;

            let shell = Shell::<Message>::new(event_queue.handle(), wl, wgpu);
            let mut client = Client::new(shell, &event_queue.handle());

            let mut pool = Pool::<Message>::new(None::<fn(Report) -> Message>);
            pool.submit(task).await;

            let mut model = self;

            std::thread::spawn(move || {
                let elements = model.view().root_widgets();
                if let Err(e) = client.widgets.reconciliate(elements, &mut client.shell) {
                    println!("Error {}", e);
                }

                'main: loop {
                    if let Err(e) = event_queue.blocking_dispatch(&mut client) {
                        println!("Error while dispatching events: {}", e);
                        break 'main;
                    }

                    let mut update = false;

                    'inner: loop {
                        let msg_opt = client
                            .msg
                            .pop()
                            .map(Ok::<Message, Report>)
                            .or_else(|| pool.pop().ok().map(Ok::<Message, Report>));

                        update = update || msg_opt.is_some();

                        match msg_opt {
                            Some(Ok(msg)) => match model.controller(msg).kind {
                                TaskKind::None => continue 'inner,
                                TaskKind::Stop => break 'main,
                                TaskKind::Reset => {
                                    model = T::default();
                                    update = true;
                                }
                                TaskKind::Runnable(task) => {
                                    pool.bsubmit(task);
                                }
                            },
                            _ => break 'inner,
                        }
                    }

                    if client.shell.surfaces() == 0 {
                        break 'main;
                    }

                    if update {
                        let elements = model.view().root_widgets();
                        if let Err(e) = client.widgets.reconciliate(elements, &mut client.shell) {
                            println!("Error {}", e);
                        }
                    }

                    if let Err(e) = client.widgets.draw(&mut client.shell) {
                        println!("Error {}", e);
                    }
                }
            });

            tokio::signal::ctrl_c().await?;

            Ok(())
        }
    }

    pub trait TaskedApplicationWithErrorHandling<Message> {
        fn run(
            self,
            task: RunnableTask<Message>,
            on_error: impl Fn(Report) -> Message + 'static + Send + Sync,
        ) -> impl Future<Output = Result<()>>;
    }

    impl<T, Message: 'static + Send + Sync> TaskedApplicationWithErrorHandling<Message> for T
    where
        T: 'static + Controller<Message> + View<Message> + Default + Send + Sync,
    {
        async fn run(
            self,
            task: RunnableTask<Message>,
            on_error: impl Fn(Report) -> Message + 'static + Send + Sync,
        ) -> Result<()> {
            let wgpu = Wgpu::new();
            let (wl, mut event_queue) = Wl::new()?;

            let shell = Shell::<Message>::new(event_queue.handle(), wl, wgpu);
            let mut client = Client::new(shell, &event_queue.handle());

            let mut pool = Pool::<Message>::new(Some(on_error));
            pool.submit(task).await;

            let mut model = self;

            std::thread::spawn(move || {
                let elements = model.view().root_widgets();
                if let Err(e) = client.widgets.reconciliate(elements, &mut client.shell) {
                    println!("Error {}", e);
                }

                'main: loop {
                    if let Err(e) = event_queue.blocking_dispatch(&mut client) {
                        println!("Error while dispatching events: {}", e);
                        break 'main;
                    }

                    let mut update = false;

                    'inner: loop {
                        let msg_opt = client
                            .msg
                            .pop()
                            .map(Ok::<Message, Report>)
                            .or_else(|| pool.pop().ok().map(Ok::<Message, Report>));

                        update = update || msg_opt.is_some();

                        match msg_opt {
                            Some(Ok(msg)) => match model.controller(msg).kind {
                                TaskKind::None => continue 'inner,
                                TaskKind::Stop => break 'main,
                                TaskKind::Reset => {
                                    model = T::default();
                                    update = true;
                                }
                                TaskKind::Runnable(task) => {
                                    pool.bsubmit(task);
                                }
                            },
                            _ => break 'inner,
                        }
                    }

                    if client.shell.surfaces() == 0 {
                        break 'main;
                    }

                    if update {
                        let elements = model.view().root_widgets();
                        if let Err(e) = client.widgets.reconciliate(elements, &mut client.shell) {
                            println!("Error {}", e);
                        }
                    }

                    // I don't know why exactly, but drawing will wake up the event queue...
                    // But this is really useful so i'll just keep it
                    if let Err(e) = client.widgets.draw(&mut client.shell) {
                        println!("Error {}", e);
                    }
                }
            });

            tokio::signal::ctrl_c().await?;

            Ok(())
        }
    }
}
