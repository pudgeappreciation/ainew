mod profile_page;

use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditAttachments, EditInteractionResponse,
};

use crate::{
    discord::{bot::Bot, commands::utilities::pagination},
    global::draw_profile::DrawProfile,
};

pub use profile_page::profile_page;

pub async fn init(ctx: &Context, command: &CommandInteraction) {
    let message = CreateInteractionResponseMessage::new()
        .content("")
        .ephemeral(true);
    let initial_response = CreateInteractionResponse::Defer(message);

    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");
    }
}

pub async fn update_pagination(
    page_index: usize,
    bot: &Bot,
    ctx: &Context,
    command: &CommandInteraction,
) {
    let profiles = DrawProfile::get_available(command.user.id, &bot.database)
        .await
        .unwrap_or_default();

    let builder = EditInteractionResponse::new()
        .content("Loading...")
        .embeds(Vec::new())
        .components(vec![pagination::buttons(page_index, profiles.len(), false)])
        .attachments(EditAttachments::new());

    _ = command
        .edit_response(&ctx.http, builder)
        .await
        .map(|message| (message, page_index));
}
