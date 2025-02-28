use std::time::Duration;

use serenity::all::{
    CommandInteraction, ComponentInteractionCollector, Context, InputTextStyle, MessageId,
};
use serenity::futures::StreamExt;

use crate::discord::bot::Bot;
use crate::discord::commands::image_info::respond;
use crate::discord::commands::utilities::copy_modal;
use crate::global::draw_request::DrawRequest;

async fn get_draw_request<T>(message_ids: T, bot: &Bot) -> Option<DrawRequest>
where
    T: Iterator<Item = MessageId>,
{
    for message_id in message_ids {
        if let Ok(Some(draw_request)) =
            DrawRequest::get_from_message_id(message_id, &bot.database).await
        {
            return Some(draw_request);
        }
    }

    None
}

pub async fn handle(bot: &Bot, ctx: Context, command: CommandInteraction) {
    let message_ids = command
        .data
        .resolved
        .messages
        .iter()
        .flat_map(|message| {
            [
                Some(message.0.clone()),
                message
                    .1
                    .message_reference
                    .as_ref()
                    .map(|message| message.message_id)
                    .flatten(),
            ]
        })
        .filter_map(|message_id| message_id)
        .collect::<Vec<_>>();

    let Some(draw_request) = get_draw_request(message_ids.into_iter(), bot).await else {
        respond::no_data(&ctx, &command).await;

        return;
    };

    let Ok(message) = respond::request_data(draw_request.clone(), &ctx, &command).await else {
        return;
    };

    let mut interaction_stream = ComponentInteractionCollector::new(&ctx.shard)
        .timeout(Duration::from_secs(60 * 10))
        .message_id(message.id)
        .stream();

    while let Some(interaction) = interaction_stream.next().await {
        if let Some(_) = copy_modal::matches(interaction.data.custom_id.as_str()) {
            copy_modal::handle(
                &ctx,
                format!("/draw{}", draw_request.to_command_options()),
                InputTextStyle::Paragraph,
                &interaction,
            )
            .await;
        }
    }
}
