use std::{sync::Arc, time::Duration};

use smithay_client_toolkit::reexports::client::EventQueue;
use tokio::sync::Mutex;

pub use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Events {
    StateMightHaveChanged,
    DispatchEvents,
}

pub struct Views<T: 'static, M: Send + Sync + 'static> {
    handles: Vec<Box<dyn Fn(&T) -> View<M>>>,

    wayland: Wayland,
    event_queue: EventQueue<Wayland>,

    views: Vec<View<M>>,

    sender: tokio::sync::mpsc::UnboundedSender<Events>,
    receiver: tokio::sync::mpsc::UnboundedReceiver<Events>,
}

pub type ViewsSender = tokio::sync::mpsc::UnboundedSender<Events>;

impl<T: 'static, M: Send + Sync + 'static> Views<T, M> {
    pub fn new(views: Vec<impl Fn(&T) -> View<M> + 'static>) -> Result<Self> {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        let (wayland, event_queue) = Wayland::new()?;

        Ok(Views {
            handles: views
                .into_iter()
                .map(|v| Box::new(v) as Box<dyn Fn(&T) -> View<M>>)
                .collect(),
            views: Vec::new(),

            wayland,
            event_queue,

            sender,
            receiver,
        })
    }

    pub fn sender(&self) -> tokio::sync::mpsc::UnboundedSender<Events> {
        self.sender.clone()
    }

    pub fn send(&self, event: Events) -> Result<()> {
        self.sender.send(event).map_err(eyre::Report::msg)
    }

    pub fn call(&self, state: &T) -> Vec<View<M>> {
        self.handles.iter().map(|v| v(state)).collect()
    }

    pub async fn run(mut self, handle: Arc<Mutex<T>>) -> Result<()> {
        self.send(Events::DispatchEvents)?;

        while let Some(event) = self.receiver.recv().await {
            match event {
                Events::StateMightHaveChanged => {
                    let state = handle.lock().await;
                    self.views = self.handles.iter().map(|v| v(&state)).collect();
                }
                Events::DispatchEvents => {
                    self.wayland.dispatch(&mut self.event_queue)?;

                    let sender = self.sender();
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_millis(33)).await;
                        sender.send(Events::DispatchEvents).unwrap_or_else(|_| {
                            eprintln!("Failed to send DispatchEvents");
                        });
                    });
                }
            }
        }

        Ok(())
    }
}

pub struct View<M: Send + Sync + 'static> {
    _marker: std::marker::PhantomData<M>,
}

impl<M: Send + Sync + 'static> View<M> {
    pub fn new(_: Anchor, _: (u32, u32), _: Element<M>) -> Self {
        View {
            _marker: std::marker::PhantomData,
        }
    }
}
