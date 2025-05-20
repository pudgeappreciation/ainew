use std::time::Duration;

use serenity::{
    all::{
        CommandInteraction, ComponentInteractionCollector, Context, InputTextStyle, ResolvedOption,
    },
    futures::StreamExt,
};

use crate::{
    discord::{
        bot::Bot,
        commands::{
            draw_profile::list::active,
            option,
            utilities::{copy_modal, pagination},
        },
    },
    global::draw_profile::DrawProfile,
};

use super::respond;

pub async fn handle<'a>(
    bot: &Bot,
    ctx: &Context,
    options: &'a Vec<ResolvedOption<'a>>,
    command: &CommandInteraction,
) {
    respond::init(&ctx, &command).await;

    let initial_page_number = option::get_int("page", options.iter()).unwrap_or(1) as usize;
    let initial_page_index = initial_page_number - 1;

    let (message, mut page_index) =
        match respond::profile_page(initial_page_index as usize, &bot, &ctx, &command).await {
            Ok(message) => message,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

    let mut interaction_stream = ComponentInteractionCollector::new(&ctx.shard)
        .timeout(Duration::from_secs(60 * 10))
        .message_id(message.id)
        .stream();

    while let Some(interaction) = interaction_stream.next().await {
        if let Some(new_index) =
            pagination::matches(&interaction, page_index, bot.loras.read().await.len())
        {
            _ = interaction.defer(&ctx.http).await;
            page_index = new_index;
            respond::update_pagination(page_index, &bot, &ctx, &command).await;
            _ = respond::profile_page(page_index, &bot, &ctx, &command).await;
        } else if let Some(profile) = copy_modal::matches(interaction.data.custom_id.as_str()) {
            let maybe_profile = DrawProfile::get(command.user.id, &profile, &bot.database).await;
            let Ok(Some(profile)) = maybe_profile else {
                continue;
            };

            copy_modal::handle(
                &ctx,
                format!("/profile set\n{}", profile.to_command_options()),
                InputTextStyle::Paragraph,
                &interaction,
            )
            .await;
        } else if let Some(profile) = active::matches(interaction.data.custom_id.as_str()) {
            active::handle(ctx, &interaction).await;
            _ = DrawProfile::set_active(Some(&profile), command.user.id, &bot.database).await;
            _ = respond::profile_page(page_index, &bot, &ctx, &command).await;
        }
    }

    println!("interaction loop ended");
}
