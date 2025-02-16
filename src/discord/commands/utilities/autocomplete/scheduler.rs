use serenity::all::CreateAutocompleteResponse;

use crate::discord::bot::Bot;

pub async fn handle(bot: &Bot) -> CreateAutocompleteResponse {
    let mut response = CreateAutocompleteResponse::new();

    for scheduler in bot.schedulers.read().await.iter() {
        response = response.add_string_choice(scheduler.name.clone(), scheduler.name.clone());
    }

    response
}
