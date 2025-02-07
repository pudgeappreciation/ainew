use tokio::sync::broadcast::{self, error::RecvError, Receiver, Sender};

use crate::global::responses::Response;

#[derive(Clone)]
pub struct RespondToMessage(Sender<Response>);

impl RespondToMessage {
    pub fn respond(&self, response: Response) {
        _ = self.0.send(response);
    }
}

pub struct RespondToMessageReceiver(Receiver<Response>);

impl Clone for RespondToMessageReceiver {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}

impl RespondToMessageReceiver {
    pub async fn next(&mut self) -> Result<Response, RecvError> {
        self.0.recv().await
    }
}

pub fn make() -> (RespondToMessage, RespondToMessageReceiver) {
    let (sender, receiver) = broadcast::channel::<Response>(32);

    (RespondToMessage(sender), RespondToMessageReceiver(receiver))
}
