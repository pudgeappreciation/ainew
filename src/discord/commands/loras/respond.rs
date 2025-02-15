use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditAttachments, EditInteractionResponse,
};

use crate::discord::{
    bot::Bot,
    commands::utilities::{copy_modal, favorites, pagination},
};

pub async fn init(ctx: &Context, command: &CommandInteraction) {
    let message = CreateInteractionResponseMessage::new()
        .content("Loading loras...")
        .ephemeral(true);
    let initial_response = CreateInteractionResponse::Defer(message);

    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");
    }
}

pub async fn lora_page(
    page_index: usize,
    bot: &Bot,
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<(serenity::all::Message, usize), serenity::Error> {
    let loras = bot.loras.read().await;
    let page = pagination::page(&loras, page_index);

    let mut attachments = EditAttachments::new();
    for attachment in page.iter().filter_map(|lora| lora.attachment()) {
        attachments = attachments.add(attachment);
    }

    let embeds = page.iter().map(|lora| lora.embed()).collect();

    let builder = EditInteractionResponse::new()
        .content("")
        .embeds(embeds)
        .components(vec![
            copy_modal::buttons(&page).await,
            favorites::buttons(page, command.user.id, &bot.database).await,
            pagination::buttons(page_index, loras.len(), false),
        ])
        .attachments(attachments);

    command
        .edit_response(&ctx.http, builder)
        .await
        .map(|message| (message, page_index))
}

pub async fn update_favorites(
    page_index: usize,
    bot: &Bot,
    ctx: &Context,
    command: &CommandInteraction,
) {
    let loras = bot.loras.read().await;
    let page = pagination::page(&loras, page_index);

    let builder = EditInteractionResponse::new().components(vec![
        copy_modal::buttons(&page).await,
        favorites::buttons(
            pagination::page(&loras, page_index),
            command.user.id,
            &bot.database,
        )
        .await,
        pagination::buttons(page_index, loras.len(), false),
    ]);

    _ = command
        .edit_response(&ctx.http, builder)
        .await
        .map(|message| (message, page_index));
}

pub async fn update_pagination(
    page_index: usize,
    bot: &Bot,
    ctx: &Context,
    command: &CommandInteraction,
) {
    let loras = bot.loras.read().await;

    let builder = EditInteractionResponse::new()
        .content("Loading...")
        .embeds(Vec::new())
        .components(vec![pagination::buttons(page_index, loras.len(), false)])
        .attachments(EditAttachments::new());

    _ = command
        .edit_response(&ctx.http, builder)
        .await
        .map(|message| (message, page_index));
}
