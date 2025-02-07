use serenity::all::{
    ChannelId, Context, CreateAttachment, CreateMessage, MessageBuilder, MessageId,
    MessageReference, MessageReferenceKind, UserId,
};
use tokio::sync::broadcast::{self, Receiver, Sender};

use crate::global::draw_response::DrawResponse;

#[derive(Debug, Clone)]
pub enum Response {
    StartingDrawing {
        channel_id: ChannelId,
        user_id: UserId,
        message_id: MessageId,
    },
    DrawingResponse {
        response: DrawResponse,
        channel_id: ChannelId,
        message_id: MessageId,
    },
}

impl Response {
    async fn handle(self, ctx: &Context) {
        match self {
            Response::StartingDrawing {
                channel_id,
                user_id,
                message_id,
            } => {
                let content = MessageBuilder::new()
                    .push("Starting work on your drawing!")
                    .mention(&user_id)
                    .build();

                _ = ctx
                    .http
                    .get_message(channel_id, message_id)
                    .await
                    .unwrap()
                    .reply_ping(&ctx.http, content)
                    .await;
            }
            Response::DrawingResponse {
                response,
                channel_id,
                message_id,
            } => {
                let message_reference =
                    MessageReference::new(MessageReferenceKind::Default, channel_id)
                        .message_id(message_id);
                let message = CreateMessage::new().reference_message(message_reference);
                let message = match response.images.into_iter().next() {
                    Some(image_data) => message
                        .content("**Your drawing is finished! :D**")
                        .add_file(CreateAttachment::bytes(image_data, "image.png")),
                    None => message.content("**No image returned :/**"),
                };
                _ = channel_id.send_message(&ctx.http, message).await;
            }
        }
    }
}

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
