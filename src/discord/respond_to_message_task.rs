use serenity::all::Context;

use crate::global::channels::respond_to_message::RespondToMessageReceiver;

pub fn start(ctx: Context, mut receiver: RespondToMessageReceiver) {
    tokio::spawn(async move {
        while let Ok(response) = receiver.next().await {
            response.handle(&ctx).await
        }
    });
}
