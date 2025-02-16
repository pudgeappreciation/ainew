use serenity::all::{CommandInteraction, Context, ResolvedOption, ResolvedValue};

use crate::discord::bot::Bot;

use super::new;

pub async fn autocomplete(bot: &Bot, ctx: Context, interaction: CommandInteraction) {
    let mut response = None;

    for option in interaction.data.options().iter() {
        response = match option {
            ResolvedOption {
                value: ResolvedValue::SubCommand(options),
                name: "new",
                ..
            } => Some(new::autocomplete(bot, options, &interaction).await),
            _ => None,
        };
    }

    if let Some(option) = response {
        _ = interaction
            .create_response(
                &ctx.http,
                serenity::all::CreateInteractionResponse::Autocomplete(option),
            )
            .await;
    }
}
