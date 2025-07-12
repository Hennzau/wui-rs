use wayland_backend::client::ObjectId;

use crate::prelude::*;

pub enum Request<Message: 'static + Send + Sync> {
    Nothing,

    Distribute { id: Option<ObjectId>, event: Event },
    Close { id: ObjectId },
    Create { view: View<Message> },
}

#[derive(Debug, Clone)]
pub enum Response {
    Success,
    Failed(String),

    NotImplemented,
}

pub(crate) struct Query<Message: 'static + Send + Sync> {
    pub(crate) request: Request<Message>,
    pub(crate) response: Option<tokio::sync::oneshot::Sender<Response>>,
}

#[derive(Debug)]
pub struct Client<Message: 'static + Send + Sync> {
    pub(crate) sender: tokio::sync::mpsc::UnboundedSender<Query<Message>>,
}

impl<Message: 'static + Send + Sync> Clone for Client<Message> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

impl<Message: 'static + Send + Sync> Client<Message> {
    pub(crate) fn new() -> (Self, Server<Message>) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        (Self { sender }, Server { receiver })
    }

    pub(crate) fn send_no_result(&self, request: Request<Message>) {
        if let Err(e) = self.send(request) {
            eprintln!("Failed to send request: {}", e);
        }
    }

    pub(crate) fn send(&self, request: Request<Message>) -> Result<()> {
        let query = Query {
            request,
            response: None,
        };

        self.sender.send(query).map_err(Report::msg)
    }

    pub async fn query(&self, request: Request<Message>) -> Result<Response> {
        let (response_sender, response_receiver) = tokio::sync::oneshot::channel();

        let query = Query {
            request,
            response: Some(response_sender),
        };

        self.sender
            .send(query)
            .map_err(|e| Report::msg(format!("Failed to send query: {}", e)))?;

        response_receiver
            .await
            .map_err(|_| Report::msg("Response channel closed"))
    }
}

pub(crate) struct Server<Message: 'static + Send + Sync> {
    pub(crate) receiver: tokio::sync::mpsc::UnboundedReceiver<Query<Message>>,
}

impl<Message: 'static + Send + Sync> Server<Message> {
    pub(crate) async fn recv(&mut self) -> Result<Query<Message>> {
        self.receiver
            .recv()
            .await
            .ok_or_else(|| Report::msg("Channel closed"))
    }
}
