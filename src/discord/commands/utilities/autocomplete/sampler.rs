use serenity::all::CreateAutocompleteResponse;

use crate::discord::bot::Bot;

pub async fn handle(bot: &Bot) -> CreateAutocompleteResponse {
    let mut response = CreateAutocompleteResponse::new();

    for sampler in bot.samplers.read().await.iter() {
        response = response.add_string_choice(sampler.name.clone(), sampler.name.clone());
    }

    response
}
