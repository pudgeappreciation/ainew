use serenity::all::{CommandInteraction, Context, EditAttachments, EditInteractionResponse};

use crate::discord::{
    bot::Bot,
    commands::utilities::{copy_modal, favorites, pagination},
};

fn empty_response() -> EditInteractionResponse {
    EditInteractionResponse::new()
        .content("No models registered")
        .embeds(Vec::new())
        .components(Vec::new())
        .clear_attachments()
}

async fn content(
    page_index: usize,
    bot: &Bot,
    command: &CommandInteraction,
) -> EditInteractionResponse {
    let models = bot.models.read().await;
    let Some(page) = pagination::page(models.iter(), page_index) else {
        return empty_response();
    };

    let mut attachments = EditAttachments::new();
    for attachment in page.iter().filter_map(|model| model.attachment()) {
        attachments = attachments.add(attachment);
    }

    let embeds = page.iter().map(|model| model.embed()).collect();

    EditInteractionResponse::new()
        .content("")
        .embeds(embeds)
        .components(vec![
            copy_modal::buttons(&page).await,
            favorites::buttons(&page, command.user.id, &bot.database).await,
            pagination::buttons(page_index, models.len(), false),
        ])
        .attachments(attachments)
}

pub async fn model_page(
    page_index: usize,
    bot: &Bot,
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<(serenity::all::Message, usize), serenity::Error> {
    command
        .edit_response(&ctx.http, content(page_index, bot, command).await)
        .await
        .map(|message| (message, page_index))
}
