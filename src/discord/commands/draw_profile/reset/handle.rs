use serenity::all::{CommandInteraction, Context, ResolvedOption, ResolvedValue, UserId};

use crate::{
    discord::{bot::Bot, commands::option},
    global::draw_profile::DrawProfile,
};

use super::respond;

pub async fn handle_inner<'a>(
    bot: &Bot,
    user_id: UserId,
    options: &'a Vec<ResolvedOption<'a>>,
) -> Result<&'a str, ()> {
    let Some(profile_name) = option::get_string("profile_name", options.iter()) else {
        return Err(());
    };

    let Ok(profile) = DrawProfile::get(user_id, profile_name, &bot.database).await else {
        return Err(());
    };

    let Some(mut profile) = profile else {
        return Ok(profile_name);
    };

    for option in options.iter() {
        match option {
            ResolvedOption {
                value: ResolvedValue::Boolean(true),
                name: "prompt_head",
                ..
            } => profile.options.prompt_head = None,
            ResolvedOption {
                value: ResolvedValue::Boolean(true),
                name: "prompt_tail",
                ..
            } => profile.options.prompt_tail = None,
            ResolvedOption {
                value: ResolvedValue::Boolean(true),
                name: "negative_prompt_head",
                ..
            } => profile.options.negative_prompt_head = None,
            ResolvedOption {
                value: ResolvedValue::Boolean(true),
                name: "negative_prompt_tail",
                ..
            } => profile.options.negative_prompt_tail = None,
            ResolvedOption {
                value: ResolvedValue::Boolean(true),
                name: "steps",
                ..
            } => profile.options.steps = None,
            ResolvedOption {
                value: ResolvedValue::Boolean(true),
                name: "clip_skip",
                ..
            } => profile.options.clip_skip = None,
            ResolvedOption {
                value: ResolvedValue::Boolean(true),
                name: "cfg_scale",
                ..
            } => profile.options.cfg_scale = None,
            ResolvedOption {
                value: ResolvedValue::Boolean(true),
                name: "sampler",
                ..
            } => profile.options.sampler = None,
            ResolvedOption {
                value: ResolvedValue::Boolean(true),
                name: "scheduler",
                ..
            } => profile.options.scheduler = None,
            ResolvedOption {
                value: ResolvedValue::Boolean(true),
                name: "model",
                ..
            } => profile.options.model = None,
            ResolvedOption {
                value: ResolvedValue::Boolean(true),
                name: "size",
                ..
            } => {
                profile.options.width = None;
                profile.options.height = None;
            }
            _ => {}
        }
    }

    return profile.save(&bot.database).await.map(|_| profile_name);
}

pub async fn handle<'a>(
    bot: &Bot,
    ctx: &Context,
    options: &'a Vec<ResolvedOption<'a>>,
    interaction: &CommandInteraction,
) {
    match handle_inner(bot, interaction.user.id, options).await {
        Ok(profile_name) => respond::success(profile_name.to_string(), ctx, interaction).await,
        Err(_) => respond::failed(ctx, interaction).await,
    };
}
