use std::time::Duration;

use serenity::all::{CommandInteraction, ComponentInteractionCollector};
use serenity::futures::StreamExt;
use serenity::prelude::*;

use crate::discord::bot::Bot;
use crate::discord::commands::utilities::{copy_modal, pagination};

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
        if let Some(new_index) =
            pagination::matches(&interaction, page_index, bot.models.read().await.len() - 1)
        {
            _ = interaction.defer(&ctx.http).await;
            page_index = new_index;
            pagination::loading(page_index, bot.models.read().await.len(), &ctx, &command).await;
            _ = respond::model_page(page_index, &bot.models, &ctx, &command).await;
        } else if let Some(model) = copy_modal::matches(interaction.data.custom_id.as_str()) {
            copy_modal::handle(&ctx, model, &interaction).await;
        }
    }

    println!("interaction loop ended");
}
