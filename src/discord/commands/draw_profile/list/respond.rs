use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditAttachments, EditInteractionResponse,
};

use crate::{
    discord::{
        bot::Bot,
        commands::utilities::{copy_modal, pagination},
    },
    global::draw_profile::DrawProfile,
};

use super::active;

pub async fn init(ctx: &Context, command: &CommandInteraction) {
    let message = CreateInteractionResponseMessage::new()
        .content("")
        .ephemeral(true);
    let initial_response = CreateInteractionResponse::Defer(message);

    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");
    }
}

pub async fn profile_page(
    page_index: usize,
    bot: &Bot,
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<(serenity::all::Message, usize), serenity::Error> {
    let profiles = DrawProfile::get_available(command.user.id, &bot.database)
        .await
        .unwrap_or_else(|_| Vec::new());
    let page = pagination::page(&profiles, page_index);

    let embeds = page.iter().map(|profile| profile.embed()).collect();

    let builder = EditInteractionResponse::new()
        .content("")
        .embeds(embeds)
        .components(vec![
            copy_modal::buttons(&page).await,
            active::buttons(page).await,
            pagination::buttons(page_index, profiles.len(), false),
        ]);

    command
        .edit_response(&ctx.http, builder)
        .await
        .map(|message| (message, page_index))
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
