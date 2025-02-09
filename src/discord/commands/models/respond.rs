use serenity::all::{
    ButtonStyle, CommandInteraction, Context, CreateActionRow, CreateButton,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditAttachments,
    EditInteractionResponse, ReactionType,
};

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

fn pagination_buttons(page_index: u32, page: Page) -> Vec<CreateButton> {
    vec![
        CreateButton::new("model-page:first")
            .emoji(ReactionType::Unicode(String::from("⏪")))
            .style(ButtonStyle::Success)
            .disabled(matches!(page, Page::First | Page::Only)),
        CreateButton::new("model-page:previous")
            .emoji(ReactionType::Unicode(String::from("◀️")))
            .style(ButtonStyle::Success)
            .disabled(matches!(page, Page::First | Page::Only)),
        CreateButton::new("stop")
            .label(format!("{}", page_index + 1))
            .style(ButtonStyle::Secondary)
            .disabled(true),
        CreateButton::new("model-page:next")
            .emoji(ReactionType::Unicode(String::from("▶️")))
            .style(ButtonStyle::Success)
            .disabled(matches!(page, Page::Last | Page::Only)),
        CreateButton::new("model-page:last")
            .emoji(ReactionType::Unicode(String::from("⏩")))
            .style(ButtonStyle::Success)
            .disabled(matches!(page, Page::Last | Page::Only)),
    ]
}

pub async fn model_page(
    page_index: u32,
    models: &Models,
    ctx: &Context,
    command: &CommandInteraction,
) {
    let models = models.read().await;
    let mut pages = models.chunks(5).skip(page_index as usize);
    let page = pages.next().unwrap();

    let page_enum = match (page_index, pages.next()) {
        (0, Some(_)) => Page::First,
        (0, None) => Page::Only,
        (_, Some(_)) => Page::Middle,
        (_, None) => Page::Last,
    };

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
        .content("Hello, World!")
        .embeds(embeds)
        .components(vec![
            CreateActionRow::Buttons(buttons),
            CreateActionRow::Buttons(pagination_buttons(page_index, page_enum)),
        ])
        .attachments(attachments);

    _ = command.edit_response(&ctx.http, builder).await;
}
