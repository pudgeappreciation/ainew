use std::time::Duration;

use serenity::all::{
    CommandInteraction, ComponentInteractionCollector, InputTextStyle, ResolvedOption,
};
use serenity::futures::StreamExt;
use serenity::prelude::*;

use crate::discord::bot::Bot;
use crate::discord::commands::option;
use crate::discord::commands::utilities::{copy_modal, favorites, pagination};

use super::respond;

pub async fn handle<'a>(
    bot: &Bot,
    ctx: &Context,
    options: &'a Vec<ResolvedOption<'a>>,
    command: &CommandInteraction,
) {
    respond::init(&ctx, &command).await;

    let initial_page = option::get_int("page", options.iter()).unwrap_or(0) as usize;

    let (message, mut page_index) =
        match respond::lora_page(initial_page, &bot, &ctx, &command).await {
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
            _ = respond::lora_page(page_index, &bot, &ctx, &command).await;
        } else if let Some(lora) = copy_modal::matches(interaction.data.custom_id.as_str()) {
            copy_modal::handle(
                &ctx,
                format!("<lora:{}:1.0>", lora),
                InputTextStyle::Short,
                &interaction,
            )
            .await;
        } else if let Some(favorite) = favorites::matches(interaction.data.custom_id.as_str()) {
            favorites::handle(&ctx, &interaction).await;
            _ = favorite.save(command.user.id, &bot.database).await;
            respond::update_favorites(page_index, &bot, &ctx, &command).await;
            _ = respond::lora_page(page_index, &bot, &ctx, &command).await;
        }
    }

    println!("interaction loop ended");
}
