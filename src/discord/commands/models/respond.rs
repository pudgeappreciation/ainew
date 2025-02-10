use std::time::Duration;

use serenity::all::{
    ButtonStyle, CommandInteraction, ComponentInteraction, Context, CreateActionRow, CreateButton,
    CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, CreateQuickModal,
    EditAttachments, EditInteractionResponse, ReactionType,
};
use tokio::time::timeout;

use crate::global::models::Models;

pub async fn init(ctx: &Context, command: &CommandInteraction) {
    let message = CreateInteractionResponseMessage::new()
        .content("Loading models...")
        .ephemeral(true);
    let initial_response = CreateInteractionResponse::Defer(message);

    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");
    }
}

#[derive(PartialEq, Eq)]
enum Page {
    First,
    Last,
    Middle,
    Only,
}

fn pagination_buttons(page_index: usize, item_count: usize, disabled: bool) -> Vec<CreateButton> {
    let page = match (page_index, item_count > page_index + 1) {
        (0, true) => Page::First,
        (0, false) => Page::Only,
        (_, true) => Page::Middle,
        (_, false) => Page::Last,
    };

    vec![
        CreateButton::new("set-page:first")
            .emoji(ReactionType::Unicode(String::from("⏪")))
            .style(ButtonStyle::Success)
            .disabled(disabled || matches!(page, Page::First | Page::Only)),
        CreateButton::new("set-page:previous")
            .emoji(ReactionType::Unicode(String::from("◀️")))
            .style(ButtonStyle::Success)
            .disabled(disabled || matches!(page, Page::First | Page::Only)),
        CreateButton::new("stop")
            .label(format!("{}", page_index + 1))
            .style(ButtonStyle::Secondary)
            .disabled(true),
        CreateButton::new("set-page:next")
            .emoji(ReactionType::Unicode(String::from("▶️")))
            .style(ButtonStyle::Success)
            .disabled(disabled || matches!(page, Page::Last | Page::Only)),
        CreateButton::new("set-page:last")
            .emoji(ReactionType::Unicode(String::from("⏩")))
            .style(ButtonStyle::Success)
            .disabled(disabled || matches!(page, Page::Last | Page::Only)),
    ]
}

pub async fn set_model_modal(ctx: &Context, interaction: &ComponentInteraction) {
    let ctx = ctx.clone();
    let interaction = interaction.clone();

    tokio::spawn(async move {
        let Some(model) = interaction.data.custom_id.get(10..) else {
            return;
        };

        let response = interaction.quick_modal(
            &ctx,
            CreateQuickModal::new("Model name to copy (for mobile)").field(
                CreateInputText::new(
                    serenity::all::InputTextStyle::Short,
                    "Model",
                    "model-response",
                )
                .value(model),
            ),
        );

        let response = timeout(Duration::from_secs(60 * 15), response)
            .await
            .map(|r| r.ok())
            .unwrap_or_default()
            .unwrap_or_default();

        if let Some(response) = response {
            _ = response
                .interaction
                .create_response(
                    &ctx.http,
                    serenity::all::CreateInteractionResponse::Acknowledge,
                )
                .await;
        }
    });
}

pub async fn loading(
    page_index: usize,
    item_count: usize,
    ctx: &Context,
    interaction: &CommandInteraction,
) {
    _ = interaction
        .edit_response(
            &ctx.http,
            EditInteractionResponse::new()
                .embeds(Vec::new())
                .clear_attachments()
                .components(vec![CreateActionRow::Buttons(pagination_buttons(
                    page_index, item_count, true,
                ))])
                .content("Loading..."),
        )
        .await;
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

    let buttons: Vec<_> = page
        .iter()
        .enumerate()
        .map(|(i, model)| {
            let emoji = match i {
                0 => "1️⃣",
                1 => "2️⃣",
                2 => "3️⃣",
                3 => "4️⃣",
                _ => "5️⃣",
            };
            CreateButton::new(format!("set-model:{}", model.internal_name))
                .emoji(ReactionType::Unicode(String::from(emoji)))
        })
        .collect();

    let mut attachments = EditAttachments::new();
    for attachment in page.iter().filter_map(|model| model.attachment()) {
        attachments = attachments.add(attachment);
    }

    let embeds = page.iter().map(|model| model.embed()).collect();

    let builder = EditInteractionResponse::new()
        .embeds(embeds)
        .components(vec![
            CreateActionRow::Buttons(buttons),
            CreateActionRow::Buttons(pagination_buttons(page_index, models.len(), false)),
        ])
        .attachments(attachments);

    command
        .edit_response(&ctx.http, builder)
        .await
        .map(|message| (message, page_index))
}
