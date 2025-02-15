use serenity::all::{CommandInteraction, CreateAutocompleteResponse};

use crate::{
    discord::bot::Bot,
    global::{draw_request::DrawRequest, favorites},
};

pub async fn handle(bot: &Bot, interaction: &CommandInteraction) -> CreateAutocompleteResponse {
    let mut response = CreateAutocompleteResponse::new();
    let mut added = Vec::new();
    let models = bot.models.read().await;

    let favorites = favorites::get_favorites(interaction.user.id, &bot.database).await;
    for (value, name) in favorites.iter().filter_map(|favorite| {
        models
            .iter()
            .find(|model| model.internal_name == *favorite)
            .map(|model| model.name_autocomplete())
    }) {
        if added.len() >= 25 {
            return response;
        }

        let name = format!("⭐ {}", name);

        if !added.contains(&value) {
            response = response.add_string_choice(name, value.clone());
            added.push(value);
        }
    }

    let popular: Vec<_> = DrawRequest::popular_models(&bot.database).await;
    for (value, name) in popular
        .iter()
        .filter_map(|favorite| {
            models
                .iter()
                .find(|model| model.internal_name == *favorite)
                .map(|model| model.name_autocomplete())
        })
        .take(5)
    {
        if added.len() >= 25 {
            return response;
        }

        let name = format!("🔥 {}", name);

        if !added.contains(&value) {
            response = response.add_string_choice(name, value.clone());
            added.push(value);
        }
    }

    for (value, name) in models.iter().map(|model| model.name_autocomplete()) {
        if added.len() >= 25 {
            return response;
        }

        if !added.contains(&value) {
            response = response.add_string_choice(name, value.clone());
            added.push(value);
        }
    }

    response
}
