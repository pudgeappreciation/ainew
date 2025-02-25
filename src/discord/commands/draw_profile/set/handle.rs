use serenity::all::{CommandInteraction, Context, ResolvedOption};

use crate::{discord::bot::Bot, global::draw_profile::DrawProfile};

use super::respond;

pub async fn handle<'a>(
    bot: &Bot,
    ctx: &Context,
    options: &'a Vec<ResolvedOption<'a>>,
    interaction: &CommandInteraction,
) {
    let Some(mut draw_profile) = DrawProfile::new_from_command(interaction.user.id, options) else {
        respond::parse_failure(ctx, interaction).await;

        return;
    };

    if let Some(base) = DrawProfile::get(interaction.user.id, &draw_profile.name, &bot.database)
        .await
        .ok()
        .flatten()
    {
        draw_profile = draw_profile.merge(&base);
    };

    match draw_profile.save(&bot.database).await {
        Ok(_) => respond::success(draw_profile.name.clone(), ctx, interaction).await,
        Err(_) => respond::save_failure(ctx, interaction).await,
    };
}
