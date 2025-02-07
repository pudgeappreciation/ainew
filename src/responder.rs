use serenity::all::Context;
use tokio::sync::broadcast::{self, Receiver, Sender};

use crate::global::responses::Response;

#[derive(Clone)]
pub struct ResponseSender(Sender<Response>);

impl ResponseSender {
    pub fn send(&self, response: Response) {
        _ = self.0.send(response);
    }
}

pub struct ResponseReceiver(Receiver<Response>);

impl Clone for ResponseReceiver {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}

pub fn make_channel() -> (ResponseSender, ResponseReceiver) {
    let (sender, receiver) = broadcast::channel::<Response>(32);

    (ResponseSender(sender), ResponseReceiver(receiver))
}

pub fn start_responder(ctx: Context, mut receiver: ResponseReceiver) {
    tokio::spawn(async move {
        loop {
            match receiver.0.recv().await {
                Ok(response) => response.handle(&ctx).await,
                Err(_) => {}
            };
        }
    });
}
