use serenity::all::CommandInteraction;
use serenity::prelude::*;

use crate::{discord::bot::Bot, global::draw_request::DrawRequest};

use super::respond;

pub async fn handle<'a>(bot: &Bot, ctx: Context, command: CommandInteraction) {
    let Some(message_id) = respond::init(&ctx, &command).await.ok() else {
        respond::failure(&ctx, &command).await;
        return;
    };

    let request = DrawRequest::new_from_command(&command, message_id);
    let result = request.save(&bot.database);

    match result.await {
        Ok(_) => respond::success(&ctx, &command).await,
        Err(_) => respond::failure(&ctx, &command).await,
    }

    bot.draw_task.wake();
}
