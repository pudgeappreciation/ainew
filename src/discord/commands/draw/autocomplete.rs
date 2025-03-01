use serenity::all::{
    CommandInteraction, Context, CreateAutocompleteResponse, ResolvedOption, ResolvedValue,
};

use crate::discord::{bot::Bot, commands::utilities};

async fn get_options(bot: &Bot, interaction: &CommandInteraction) -> CreateAutocompleteResponse {
    for option in interaction.data.options().iter() {
        match option {
            ResolvedOption {
                value: ResolvedValue::Autocomplete { kind: _, value: _ },
                name: "model",
                ..
            } => return utilities::autocomplete::model(bot, interaction).await,
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

    CreateAutocompleteResponse::new()
}

pub async fn autocomplete(bot: &Bot, ctx: Context, interaction: CommandInteraction) {
    _ = interaction
        .create_response(
            &ctx.http,
            serenity::all::CreateInteractionResponse::Autocomplete(
                get_options(bot, &interaction).await,
            ),
        )
        .await;
}
