use std::str::FromStr;

use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    ReactionType,
};

use crate::{
    discord::commands::utilities::copy_modal::buttons_with_emoji, global::draw_request::DrawRequest,
};

pub async fn request_data(
    draw_request: DrawRequest,
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<serenity::all::Message, serenity::Error> {
    let buttons = vec![(
        draw_request.message_id.to_string(),
        ReactionType::from_str("📋").unwrap(),
    )];

    let message = CreateInteractionResponseMessage::new()
        .content("")
        .add_embed(draw_request.into())
        .components(vec![buttons_with_emoji(&buttons)])
        .ephemeral(true);
    let initial_response = CreateInteractionResponse::Message(message);

    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");

        return Err(why);
    }

    command.get_response(&ctx.http).await
}
