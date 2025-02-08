use serenity::all::Context;

use crate::global::channels::respond_to_message::RespondToMessageReceiver;

pub fn start(ctx: Context, mut receiver: RespondToMessageReceiver) {
    tokio::spawn(async move {
        while let Ok((message, channel_id)) = receiver.next().await {
            _ = channel_id.send_message(&ctx.http, message).await;
        }
    });
}
