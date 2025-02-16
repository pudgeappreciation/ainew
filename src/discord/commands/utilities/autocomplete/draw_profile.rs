use serenity::all::{CommandInteraction, CreateAutocompleteResponse};

use crate::{discord::bot::Bot, global::draw_profile::DrawProfile};

pub async fn handle(bot: &Bot, interaction: &CommandInteraction) -> CreateAutocompleteResponse {
    let mut response = CreateAutocompleteResponse::new();

    for profile in DrawProfile::get_available(interaction.user.id, &bot.database)
        .await
        .unwrap_or_else(|_| Vec::new())
    {
        response = response.add_string_choice(profile.clone(), profile);
    }

    response
}
