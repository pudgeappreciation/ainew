use serenity::all::CommandInteraction;
use serenity::prelude::*;

use crate::discord::bot::Bot;

use super::respond;

pub async fn handle<'a>(bot: &Bot, ctx: Context, command: CommandInteraction) {
    respond::init(&ctx, &command).await;

    respond::model_page(0, &bot.models, &ctx, &command).await;
}
