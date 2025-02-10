use std::time::Duration;

use serenity::all::{CommandInteraction, ComponentInteractionCollector};
use serenity::futures::StreamExt;
use serenity::prelude::*;

use crate::discord::bot::Bot;

use super::respond;

pub async fn handle<'a>(bot: &Bot, ctx: Context, command: CommandInteraction) {
    respond::init(&ctx, &command).await;

    let (message, mut page_index) = match respond::model_page(0, &bot.models, &ctx, &command).await
    {
        Ok(message) => message,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let mut interaction_stream = ComponentInteractionCollector::new(&ctx.shard)
        .timeout(Duration::from_secs(60 * 10))
        .message_id(message.id)
        .stream();

    while let Some(interaction) = interaction_stream.next().await {
        println!("{}", interaction.data.custom_id.as_str());
        if interaction.data.custom_id.starts_with("set-page:") {
            page_index = match interaction.data.custom_id.as_str() {
                "set-page:last" => (bot.models.read().await.len() - 1) / 5,
                "set-page:next" => page_index.saturating_add(1),
                "set-page:previous" => page_index.saturating_sub(1),
                _ => 0,
            };
            _ = interaction.defer(&ctx.http).await;
            respond::loading(page_index, bot.models.read().await.len(), &ctx, &command).await;
            _ = respond::model_page(page_index, &bot.models, &ctx, &command).await;
        } else if interaction.data.custom_id.starts_with("set-model:") {
            respond::set_model_modal(&ctx, &interaction).await;
        }
    }

    println!("interaction loop ended");
}
