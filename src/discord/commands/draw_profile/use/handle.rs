use serenity::all::{CommandInteraction, Context, ResolvedOption, UserId};

use crate::{
    discord::{bot::Bot, commands::option},
    global::draw_profile::DrawProfile,
};

use super::respond;

pub async fn handle_inner<'a>(
    bot: &Bot,
    user_id: UserId,
    options: &'a Vec<ResolvedOption<'a>>,
) -> Option<&'a str> {
    let profile = option::get_string("profile_name", options.iter());

    match DrawProfile::set_active(profile, user_id, &bot.database).await {
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
