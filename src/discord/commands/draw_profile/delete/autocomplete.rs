use serenity::all::{
    CommandInteraction, CreateAutocompleteResponse, ResolvedOption, ResolvedValue,
};

use crate::discord::{bot::Bot, commands::utilities};

pub async fn autocomplete<'a>(
    bot: &Bot,
    options: &Vec<ResolvedOption<'a>>,
    interaction: &CommandInteraction,
) -> CreateAutocompleteResponse {
    for option in options {
        match option {
            ResolvedOption {
                value: ResolvedValue::Autocomplete { kind: _, value: _ },
                name: "profile_name",
                ..
            } => return utilities::autocomplete::draw_profile(bot, interaction).await,
            _ => {}
        }
    }

    return CreateAutocompleteResponse::new();
}
