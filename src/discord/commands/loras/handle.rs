use std::time::Duration;

use serenity::all::{CommandInteraction, ComponentInteractionCollector};
use serenity::futures::StreamExt;
use serenity::prelude::*;

use crate::discord::bot::Bot;
use crate::discord::commands::utilities::{copy_modal, favorites, pagination};

use super::respond;

pub async fn handle<'a>(bot: &Bot, ctx: Context, command: CommandInteraction) {
    respond::init(&ctx, &command).await;

    let (message, mut page_index) = match respond::model_page(0, &bot, &ctx, &command).await {
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
        if let Some(new_index) =
            pagination::matches(&interaction, page_index, bot.loras.read().await.len())
        {
            _ = interaction.defer(&ctx.http).await;
            page_index = new_index;
            respond::update_pagination(page_index, &bot, &ctx, &command).await;
            _ = respond::model_page(page_index, &bot, &ctx, &command).await;
        } else if let Some(model) = copy_modal::matches(interaction.data.custom_id.as_str()) {
            copy_modal::handle(&ctx, model, &interaction).await;
        } else if let Some(favorite) = favorites::matches(interaction.data.custom_id.as_str()) {
            favorites::handle(&ctx, &interaction).await;
            _ = favorite.save(command.user.id, &bot.database).await;
            respond::update_favorites(page_index, &bot, &ctx, &command).await;
            _ = respond::model_page(page_index, &bot, &ctx, &command).await;
        }
    }

    println!("interaction loop ended");
}
