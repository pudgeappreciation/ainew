use super::list_with_images;

use serenity::all::{CommandInteraction, Context, ResolvedOption, ResolvedValue};

use crate::discord::bot::Bot;

pub async fn handle(bot: &Bot, ctx: Context, interaction: CommandInteraction) {
    for option in interaction.data.options().iter() {
        match option {
            ResolvedOption {
                value: ResolvedValue::SubCommand(options),
                name: "images",
                ..
            } => return list_with_images::handle(bot, &ctx, &options, &interaction).await,
            _ => {
                _ = interaction
                    .create_response(
                        &ctx.http,
                        serenity::all::CreateInteractionResponse::Acknowledge,
                    )
                    .await;

                return;
            }
        };
    }
}
