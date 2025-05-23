use serenity::all::{
    ButtonStyle, ComponentInteraction, CreateActionRow, CreateButton, ReactionType,
};

#[derive(PartialEq, Eq)]
enum Page {
    First,
    Last,
    Middle,
    Only,
}

fn max_page_index(item_count: usize) -> usize {
    item_count.saturating_sub(1) / 5
}

fn clamp_page_index(page_index: usize, item_count: usize) -> usize {
    page_index.min(max_page_index(item_count))
}

pub fn page<T>(items: T, page_index: usize) -> Option<Vec<T::Item>>
where
    T: ExactSizeIterator,
{
    let index = clamp_page_index(page_index, items.len());

    let page: Vec<_> = items.skip(index * 5).take(5).collect();

    match page.is_empty() {
        false => Some(page),
        true => None,
    }
}

pub fn buttons(page_index: usize, item_count: usize, disabled: bool) -> CreateActionRow {
    let page_index = clamp_page_index(page_index, item_count);

    let items_seen = (page_index + 1) * 5;
    let page = match (page_index, item_count <= items_seen) {
        (0, false) => Page::First,
        (0, true) => Page::Only,
        (_, false) => Page::Middle,
        (_, true) => Page::Last,
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
            .label(format!(
                "{} / {}",
                page_index + 1,
                max_page_index(item_count) + 1,
            ))
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
    item_count: usize,
) -> Option<usize> {
    let page_index = clamp_page_index(page_index, item_count);

    if !interaction.data.custom_id.as_str().starts_with("set-page:") {
        return None;
    }

    let page_index = match interaction.data.custom_id.as_str() {
        "set-page:last" => (item_count.saturating_sub(1)) / 5,
        "set-page:next" => page_index.saturating_add(1),
        "set-page:previous" => page_index.saturating_sub(1),
        _ => 0,
    };
    Some(page_index)
}
