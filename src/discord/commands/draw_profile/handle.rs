use super::{delete, list, new, r#use, reset};

use serenity::all::{CommandInteraction, Context, ResolvedOption, ResolvedValue};

use crate::discord::bot::Bot;

pub async fn handle(bot: &Bot, ctx: Context, interaction: CommandInteraction) {
    for option in interaction.data.options().iter() {
        match option {
            ResolvedOption {
                value: ResolvedValue::SubCommand(options),
                name: "delete",
                ..
            } => return delete::handle(bot, &ctx, &options, &interaction).await,
            ResolvedOption {
                value: ResolvedValue::SubCommand(options),
                name: "list",
                ..
            } => return list::handle(bot, &ctx, &options, &interaction).await,
            ResolvedOption {
                value: ResolvedValue::SubCommand(options),
                name: "new",
                ..
            } => return new::handle(bot, &ctx, &options, &interaction).await,
            ResolvedOption {
                value: ResolvedValue::SubCommand(options),
                name: "reset",
                ..
            } => return reset::handle(bot, &ctx, &options, &interaction).await,
            ResolvedOption {
                value: ResolvedValue::SubCommand(options),
                name: "use",
                ..
            } => return r#use::handle(bot, &ctx, &options, &interaction).await,
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
