mod lora_page;

use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditAttachments, EditInteractionResponse,
};

use crate::discord::{
    bot::Bot,
    commands::utilities::{copy_modal, favorites, pagination},
};

pub use lora_page::lora_page;

pub async fn init(ctx: &Context, command: &CommandInteraction) {
    let message = CreateInteractionResponseMessage::new()
        .content("Loading loras...")
        .ephemeral(true);
    let initial_response = CreateInteractionResponse::Defer(message);

    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");
    }
}

pub async fn update_favorites(
    page_index: usize,
    bot: &Bot,
    ctx: &Context,
    command: &CommandInteraction,
) {
    let loras = bot.loras.read().await;
    let Some(page) = pagination::page(loras.iter(), page_index) else {
        return;
    };

    let builder = EditInteractionResponse::new().components(vec![
        copy_modal::buttons(&page).await,
        favorites::buttons(&page, command.user.id, &bot.database).await,
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
