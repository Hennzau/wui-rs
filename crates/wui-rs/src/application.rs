use tokio::sync::mpsc::UnboundedReceiver;

pub mod channel;
pub use channel::*;

pub(crate) mod handler;
pub(crate) use handler::*;

pub(crate) mod wayland;
pub(crate) use wayland::*;

pub(crate) mod inner;
pub(crate) use inner::*;

use crate::prelude::*;

pub struct Application<State: 'static, Message: 'static + Send + Sync> {
    pub(crate) state: State,

    pub(crate) update: Box<dyn Fn(&mut State, Message)>,
    pub(crate) views: Box<dyn Fn(&State) -> Views<Message>>,

    pub(crate) receiver: UnboundedReceiver<Message>,
    pub(crate) inner: ApplicationInner<Message>,
}

impl<State: 'static, Message: 'static + Send + Sync> Application<State, Message> {
    pub fn new(
        state: State,
        update: impl Fn(&mut State, Message) + 'static,
        views: impl Fn(&State) -> Views<Message> + 'static,
    ) -> Result<Self> {
        let (receiver, inner) = ApplicationInner::new()?;

        Ok(Self {
            state,
            update: Box::new(update),
            views: Box::new(views),
            inner,
            receiver,
        })
    }

    pub fn views(&self) -> Views<Message> {
        (self.views)(&self.state)
    }

    pub async fn run(mut self) -> Result<()> {
        let client = self.inner.client.clone();

        let inner = tokio::task::spawn(async move { self.inner.run().await });

        let views = (self.views)(&self.state);
        client.query(Request::Create { views }).await?;

        let mut server = async move || -> Result<()> {
            while let Some(message) = self.receiver.recv().await {
                (self.update)(&mut self.state, message);

                let views = (self.views)(&self.state);
                client.query(Request::Create { views }).await?;
            }

            Ok(())
        };

        tokio::select! {
            res = inner => res?,
            res = server() => res,
        }
    }
}
