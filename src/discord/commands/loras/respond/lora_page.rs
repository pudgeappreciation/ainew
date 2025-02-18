use serenity::all::{CommandInteraction, Context, EditAttachments, EditInteractionResponse};

use crate::discord::{
    bot::Bot,
    commands::utilities::{copy_modal, favorites, pagination},
};

fn empty_response() -> EditInteractionResponse {
    EditInteractionResponse::new()
        .content("No loras registered")
        .embeds(Vec::new())
        .components(Vec::new())
        .clear_attachments()
}

async fn contents(
    page_index: usize,
    bot: &Bot,
    command: &CommandInteraction,
) -> EditInteractionResponse {
    let loras = bot.loras.read().await;
    let Some(page) = pagination::page(loras.iter(), page_index) else {
        return empty_response();
    };

    let mut attachments = EditAttachments::new();
    for attachment in page.iter().filter_map(|lora| lora.attachment()) {
        attachments = attachments.add(attachment);
    }

    let embeds = page.iter().map(|lora| lora.embed()).collect();

    EditInteractionResponse::new()
        .content("")
        .embeds(embeds)
        .components(vec![
            copy_modal::buttons(&page).await,
            favorites::buttons(&page, command.user.id, &bot.database).await,
            pagination::buttons(page_index, loras.len(), false),
        ])
        .attachments(attachments)
}

pub async fn lora_page(
    page_index: usize,
    bot: &Bot,
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<(serenity::all::Message, usize), serenity::Error> {
    command
        .edit_response(&ctx.http, contents(page_index, bot, command).await)
        .await
        .map(|message| (message, page_index))
}
