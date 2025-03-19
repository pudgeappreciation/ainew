use std::time::Duration;

use serenity::all::{CommandInteraction, ComponentInteractionCollector, ResolvedOption};
use serenity::futures::StreamExt;
use serenity::prelude::*;

use crate::discord::bot::Bot;
use crate::discord::commands::option;
use crate::discord::commands::utilities::pagination;

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
        }
    }

    println!("interaction loop ended");
}
