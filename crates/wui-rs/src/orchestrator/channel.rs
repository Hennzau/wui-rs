use wayland_backend::client::ObjectId;

use crate::prelude::*;

pub enum Request {
    Nothing,

    #[allow(private_interfaces)]
    ForwardEvent {
        event: ViewEvent,
        id: Option<ObjectId>,
    },

    CloseView(ObjectId),

    CreateViewLayer(ViewConfiguration),
    CreateViewWindow(ViewConfiguration),

    AttachChild {
        id: ObjectId,
        child: Box<dyn Element>,
    },

    Caching,
    Garbage,
}

#[derive(Debug, Clone)]
pub enum Response {
    Success(Option<ObjectId>),
    Failed(String),

    NotImplemented,
}

pub(crate) struct Query {
    pub(crate) request: Request,
    pub(crate) response: Option<tokio::sync::oneshot::Sender<Response>>,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) sender: tokio::sync::mpsc::UnboundedSender<Query>,
}

impl Client {
    pub(crate) fn new() -> (Self, Server) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        (Self { sender }, Server { receiver })
    }

    pub(crate) fn send_no_result(&self, request: Request) {
        if let Err(e) = self.send(request) {
            eprintln!("Failed to send request: {}", e);
        }
    }

    pub(crate) fn send(&self, request: Request) -> Result<()> {
        let query = Query {
            request,
            response: None,
        };

        self.sender.send(query).map_err(Report::msg)
    }

    pub async fn query(&self, request: Request) -> Result<Response> {
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

pub(crate) struct Server {
    pub(crate) receiver: tokio::sync::mpsc::UnboundedReceiver<Query>,
}

impl Server {
    pub(crate) async fn recv(&mut self) -> Result<Query> {
        self.receiver
            .recv()
            .await
            .ok_or_else(|| Report::msg("Channel closed"))
    }
}
