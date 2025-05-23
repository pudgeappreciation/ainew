use std::time::Duration;

use serenity::all::{
    ComponentInteraction, Context, CreateActionRow, CreateButton, CreateInputText,
    CreateQuickModal, InputTextStyle, ReactionType,
};
use tokio::time::timeout;

pub trait CopyButtonId {
    fn id(&self) -> String;
}

impl<T> CopyButtonId for &T
where
    T: CopyButtonId,
{
    fn id(&self) -> String {
        self.to_owned().id()
    }
}

impl CopyButtonId for String {
    fn id(&self) -> String {
        self.clone()
    }
}

pub async fn buttons<T>(page: &Vec<T>) -> CreateActionRow
where
    T: CopyButtonId,
{
    let buttons: Vec<_> = page
        .iter()
        .enumerate()
        .map(|(i, page_item)| {
            let emoji = match i {
                0 => "1️⃣",
                1 => "2️⃣",
                2 => "3️⃣",
                3 => "4️⃣",
                _ => "5️⃣",
            };
            CreateButton::new(format!("copy-value:{}", page_item.id()))
                .emoji(ReactionType::Unicode(String::from(emoji)))
        })
        .collect();

    CreateActionRow::Buttons(buttons)
}

pub fn buttons_with_emoji<T>(page: &Vec<(T, ReactionType)>) -> CreateActionRow
where
    T: CopyButtonId,
{
    let buttons: Vec<_> = page
        .iter()
        .map(|(page_item, emoji)| {
            CreateButton::new(format!("copy-value:{}", page_item.id())).emoji(emoji.clone())
        })
        .collect();

    CreateActionRow::Buttons(buttons)
}

pub fn matches(id: &str) -> Option<String> {
    if !id.starts_with("copy-value:") {
        return None;
    }

    id.get(11..).map(|value| value.to_string())
}

pub async fn handle(
    ctx: &Context,
    value: String,
    mode: InputTextStyle,
    interaction: &ComponentInteraction,
) {
    let ctx = ctx.clone();
    let interaction = interaction.clone();

    tokio::spawn(async move {
        let response = interaction.quick_modal(
            &ctx,
            CreateQuickModal::new("Value to copy (for mobile)")
                .field(CreateInputText::new(mode, "Value", "value-response").value(value)),
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
