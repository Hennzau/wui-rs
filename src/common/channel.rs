use crate::*;

pub struct Sender<T> {
    pub(crate) tx: tokio::sync::mpsc::Sender<T>,
}

impl<T: Send + Sync + 'static> Sender<T> {
    pub fn bsend(&self, value: T) {
        self.tx
            .blocking_send(value)
            .unwrap_or_else(|e| tracing::error!("Failed to send: {}", e));
    }

    pub async fn send(&self, value: T) {
        self.tx
            .send(value)
            .await
            .unwrap_or_else(|e| tracing::error!("Failed to send: {}", e));
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
    pub(crate) rx: tokio::sync::mpsc::Receiver<T>,
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

pub fn channel<T>(size: usize) -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = tokio::sync::mpsc::channel(size);

    (Sender { tx }, Receiver { rx })
}
