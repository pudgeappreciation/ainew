mod model;
mod sampler;
mod scheduler;
mod size;

use serenity::all::{
    CommandInteraction, Context, CreateAutocompleteResponse, ResolvedOption, ResolvedValue,
};

use crate::discord::bot::Bot;

async fn get_options(bot: &Bot, interaction: &CommandInteraction) -> CreateAutocompleteResponse {
    for option in interaction.data.options().iter() {
        match option {
            ResolvedOption {
                value: ResolvedValue::Autocomplete { kind: _, value: _ },
                name: "model",
                ..
            } => return model::handle(bot, interaction).await,
            ResolvedOption {
                value: ResolvedValue::Autocomplete { kind: _, value: _ },
                name: "size",
                ..
            } => return size::handle(),
            ResolvedOption {
                value: ResolvedValue::Autocomplete { kind: _, value: _ },
                name: "sampler",
                ..
            } => return sampler::handle(bot).await,
            ResolvedOption {
                value: ResolvedValue::Autocomplete { kind: _, value: _ },
                name: "scheduler",
                ..
            } => return scheduler::handle(bot).await,
            _ => {}
        }
    }

    return CreateAutocompleteResponse::new();
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
