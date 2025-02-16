use serenity::all::{CommandInteraction, Context, ResolvedOption, ResolvedValue, UserId};

use crate::{discord::bot::Bot, global::draw_profile::DrawProfile};

use super::respond;

pub async fn handle_inner<'a>(
    bot: &Bot,
    user_id: UserId,
    options: &'a Vec<ResolvedOption<'a>>,
) -> Result<String, ()> {
    for option in options.iter() {
        if let ResolvedOption {
            name: "profile_name",
            value: ResolvedValue::String(profile_name),
            ..
        } = option
        {
            return DrawProfile::remove(profile_name.to_string(), user_id, &bot.database)
                .await
                .map(|_| profile_name.to_string());
        };
    }

    Err(())
}

pub async fn handle<'a>(
    bot: &Bot,
    ctx: &Context,
    options: &'a Vec<ResolvedOption<'a>>,
    interaction: &CommandInteraction,
) {
    match handle_inner(bot, interaction.user.id, options).await {
        Ok(profile_name) => respond::success(profile_name, ctx, interaction).await,
        Err(_) => respond::failed(ctx, interaction).await,
    };
}
