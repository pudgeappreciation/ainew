use serenity::all::{
    ComponentInteraction, Context, CreateActionRow, CreateButton, ReactionType, UserId,
};
use sqlx::{Pool, Sqlite};

use crate::global::favorites::{self, Favorite, FavoritesString};

pub async fn buttons<'a, T>(page: T, user: UserId, database: &Pool<Sqlite>) -> CreateActionRow
where
    T: IntoIterator,
    T::Item: FavoritesString,
{
    let buttons = favorites::favorited(page.into_iter(), user, database)
        .await
        .iter()
        .map(|page_item| match page_item {
            favorites::Favorite::Favorited(item) => {
                CreateButton::new(format!("remove-favorite:{}", item.string()))
                    .emoji(ReactionType::Unicode(String::from("⭐")))
                    .style(serenity::all::ButtonStyle::Success)
            }
            favorites::Favorite::Unfavorited(item) => {
                CreateButton::new(format!("add-favorite:{}", item.string()))
                    .emoji(ReactionType::Unicode(String::from("⭐")))
                    .style(serenity::all::ButtonStyle::Secondary)
            }
        })
        .collect();

    CreateActionRow::Buttons(buttons)
}

pub fn matches(id: &str) -> Option<Favorite<String>> {
    if id.starts_with("remove-favorite:") {
        return id
            .get(16..)
            .map(|value| Favorite::Unfavorited(value.to_string()));
    }

    if id.starts_with("add-favorite:") {
        return id
            .get(13..)
            .map(|value| Favorite::Favorited(value.to_string()));
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
