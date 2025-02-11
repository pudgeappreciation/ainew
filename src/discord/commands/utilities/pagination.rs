use serenity::all::{
    ButtonStyle, CommandInteraction, ComponentInteraction, Context, CreateActionRow, CreateButton,
    EditInteractionResponse, ReactionType,
};

#[derive(PartialEq, Eq)]
enum Page {
    First,
    Last,
    Middle,
    Only,
}

pub fn buttons(page_index: usize, item_count: usize, disabled: bool) -> CreateActionRow {
    let page = match (page_index, item_count > page_index + 1) {
        (0, true) => Page::First,
        (0, false) => Page::Only,
        (_, true) => Page::Middle,
        (_, false) => Page::Last,
    };

    CreateActionRow::Buttons(vec![
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
    ])
}

pub fn matches(
    interaction: &ComponentInteraction,
    page_index: usize,
    value_count: usize,
) -> Option<usize> {
    if !interaction.data.custom_id.as_str().starts_with("set-page:") {
        return None;
    }

    let page_index = match interaction.data.custom_id.as_str() {
        "set-page:last" => value_count / 5,
        "set-page:next" => page_index.saturating_add(1),
        "set-page:previous" => page_index.saturating_sub(1),
        _ => 0,
    };
    Some(page_index)
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
                .components(vec![buttons(page_index, item_count, true)])
                .content("Loading..."),
        )
        .await;
}
