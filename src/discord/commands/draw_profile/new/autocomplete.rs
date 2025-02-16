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
            ResolvedOption {
                value: ResolvedValue::Autocomplete { kind: _, value: _ },
                name: "model",
                ..
            } => return utilities::autocomplete::model(bot, interaction).await,
            ResolvedOption {
                value: ResolvedValue::Autocomplete { kind: _, value: _ },
                name: "size",
                ..
            } => return utilities::autocomplete::size(),
            ResolvedOption {
                value: ResolvedValue::Autocomplete { kind: _, value: _ },
                name: "sampler",
                ..
            } => return utilities::autocomplete::sampler(bot).await,
            ResolvedOption {
                value: ResolvedValue::Autocomplete { kind: _, value: _ },
                name: "scheduler",
                ..
            } => return utilities::autocomplete::scheduler(bot).await,
            _ => {}
        }
    }

    return CreateAutocompleteResponse::new();
}
