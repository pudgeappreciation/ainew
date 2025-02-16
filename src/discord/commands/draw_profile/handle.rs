use serenity::all::{CommandInteraction, Context};

use crate::discord::bot::Bot;

pub async fn handle<'a>(bot: &Bot, ctx: Context, interaction: CommandInteraction) {
    for option in interaction.data.options().iter() {
        match option {
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
