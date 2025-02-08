use serenity::all::{ChannelId, CreateMessage};
use tokio::sync::broadcast::{self, error::RecvError, Receiver, Sender};

#[derive(Clone)]
pub struct RespondToMessage(Sender<(CreateMessage, ChannelId)>);

impl RespondToMessage {
    pub fn send(&self, response: CreateMessage, channel: ChannelId) {
        _ = self.0.send((response, channel));
    }
}

pub struct RespondToMessageReceiver(Receiver<(CreateMessage, ChannelId)>);

impl Clone for RespondToMessageReceiver {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}

impl RespondToMessageReceiver {
    pub async fn next(&mut self) -> Result<(CreateMessage, ChannelId), RecvError> {
        self.0.recv().await
    }
}

pub fn make() -> (RespondToMessage, RespondToMessageReceiver) {
    let (sender, receiver) = broadcast::channel::<(CreateMessage, ChannelId)>(32);

    (RespondToMessage(sender), RespondToMessageReceiver(receiver))
}
