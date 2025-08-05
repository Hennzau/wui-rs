use crate::prelude::*;

pub struct Sender<T> {
    pub(crate) tx: tokio::sync::mpsc::UnboundedSender<T>,
}

impl<T: Send + Sync + 'static> Sender<T> {
    pub fn send(&self, value: T) {
        self.tx
            .send(value)
            .unwrap_or_else(|e| tracing::error!("Failed to send: {}", e));
    }

    pub fn rsend(&self, value: T) -> Result<()> {
        self.tx.send(value).map_err(Report::msg)
    }
}

impl<T: Send + Sync + 'static> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

pub struct Receiver<T> {
    pub(crate) rx: tokio::sync::mpsc::UnboundedReceiver<T>,
}

impl<T> Receiver<T> {
    pub async fn recv(&mut self) -> Result<T> {
        self.rx
            .recv()
            .await
            .ok_or_else(|| Report::msg("Receiver closed"))
    }

    pub fn try_recv(&mut self) -> Result<T> {
        self.rx.try_recv().map_err(Report::msg)
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    (Sender { tx }, Receiver { rx })
}
