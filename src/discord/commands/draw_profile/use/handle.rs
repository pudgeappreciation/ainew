use serenity::all::{CommandInteraction, Context, ResolvedOption, ResolvedValue, UserId};

use crate::{discord::bot::Bot, global::draw_profile::DrawProfile};

use super::respond;

pub async fn handle_inner<'a>(
    bot: &Bot,
    user_id: UserId,
    options: &'a Vec<ResolvedOption<'a>>,
) -> Option<String> {
    let mut profile = None;

    for option in options.iter() {
        if let ResolvedOption {
            name: "profile_name",
            value: ResolvedValue::String(profile_name),
            ..
        } = option
        {
            profile = Some(profile_name.to_string());
        };
    }

    match DrawProfile::set_active(profile.clone(), user_id, &bot.database).await {
        Ok(_) => profile,
        Err(_) => None,
    }
}

pub async fn handle<'a>(
    bot: &Bot,
    ctx: &Context,
    options: &'a Vec<ResolvedOption<'a>>,
    interaction: &CommandInteraction,
) {
    match handle_inner(bot, interaction.user.id, options).await {
        Some(profile_name) => respond::set_profile(profile_name, ctx, interaction).await,
        None => respond::cleared_active_profile(ctx, interaction).await,
    };
}
