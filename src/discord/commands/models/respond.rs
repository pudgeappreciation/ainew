use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditAttachments, EditInteractionResponse,
};

use crate::{
    discord::commands::utilities::{copy_modal, pagination},
    global::models::base_model::Models,
};

pub async fn init(ctx: &Context, command: &CommandInteraction) {
    let message = CreateInteractionResponseMessage::new()
        .content("Loading models...")
        .ephemeral(true);
    let initial_response = CreateInteractionResponse::Defer(message);

    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");
    }
}

pub async fn model_page(
    page_index: usize,
    models: &Models,
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<(serenity::all::Message, usize), serenity::Error> {
    let models = models.read().await;
    let pages: Vec<_> = models.chunks(5).collect();
    let page = pages.get(page_index).expect("could not get pages");

    let mut attachments = EditAttachments::new();
    for attachment in page.iter().filter_map(|model| model.attachment()) {
        attachments = attachments.add(attachment);
    }

    let embeds = page.iter().map(|model| model.embed()).collect();

    let builder = EditInteractionResponse::new()
        .embeds(embeds)
        .components(vec![
            copy_modal::buttons(page).await,
            pagination::buttons(page_index, models.len(), false),
        ])
        .attachments(attachments);

    command
        .edit_response(&ctx.http, builder)
        .await
        .map(|message| (message, page_index))
}
