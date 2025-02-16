use serenity::all::{ComponentInteraction, Context, CreateActionRow, CreateButton, ReactionType};

use crate::global::draw_profile::DrawProfile;

pub async fn buttons(page: Vec<&DrawProfile>) -> CreateActionRow {
    let buttons = page
        .iter()
        .map(|item| match item.active {
            true => CreateButton::new(format!("set-inactive:{}", item.name))
                .emoji(ReactionType::Unicode(String::from("🔘")))
                .style(serenity::all::ButtonStyle::Success),
            false => CreateButton::new(format!("set-active:{}", item.name))
                .emoji(ReactionType::Unicode(String::from("🔘")))
                .style(serenity::all::ButtonStyle::Secondary),
        })
        .collect();

    CreateActionRow::Buttons(buttons)
}

pub fn matches(id: &str) -> Option<String> {
    if id.starts_with("set-inactive:") {
        return id.get(13..).map(|value| value.to_string());
    }

    if id.starts_with("set-active:") {
        return id.get(11..).map(|value| value.to_string());
    }

    None
}

pub async fn handle(ctx: &Context, interaction: &ComponentInteraction) {
    _ = interaction
        .create_response(
            &ctx.http,
            serenity::all::CreateInteractionResponse::Acknowledge,
        )
        .await;
}
