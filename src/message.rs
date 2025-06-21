use crate::prelude::*;

pub struct MessageQueue<M: Send + Sync + 'static> {
    sender: tokio::sync::mpsc::UnboundedSender<M>,
    receiver: tokio::sync::mpsc::UnboundedReceiver<M>,
}

pub type MessageQueueSender<M> = tokio::sync::mpsc::UnboundedSender<M>;

impl<M: Send + Sync + 'static> MessageQueue<M> {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        MessageQueue { sender, receiver }
    }

    pub fn send(&self, message: M) -> Result<()> {
        self.sender.send(message).map_err(eyre::Report::msg)
    }

    pub fn sender(&self) -> tokio::sync::mpsc::UnboundedSender<M> {
        self.sender.clone()
    }

    pub async fn receive(&mut self) -> Option<M> {
        self.receiver.recv().await
    }
}
