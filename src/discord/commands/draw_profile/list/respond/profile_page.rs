use serenity::all::{CommandInteraction, Context, EditInteractionResponse, MessageBuilder};

use crate::{
    discord::{
        bot::Bot,
        commands::{
            draw_profile::list::active,
            utilities::{copy_modal, pagination},
        },
    },
    global::draw_profile::DrawProfile,
};

fn empty_response() -> EditInteractionResponse {
    EditInteractionResponse::new()
        .content(
            MessageBuilder::new()
                .push_line_safe("You do not have any profiles.")
                .push_line_safe("")
                .push_safe("You can create a profile with the ")
                .push_mono_safe("/profile new")
                .push_safe(" command.")
                .build(),
        )
        .embeds(Vec::new())
        .components(Vec::new())
        .clear_attachments()
}

async fn contents(
    page_index: usize,
    bot: &Bot,
    command: &CommandInteraction,
) -> EditInteractionResponse {
    let Ok(profiles) = DrawProfile::get_available(command.user.id, &bot.database).await else {
        return empty_response();
    };
    let Some(page) = pagination::page(profiles.iter(), page_index) else {
        return empty_response();
    };

    let embeds = page.iter().map(|profile| profile.embed()).collect();

    EditInteractionResponse::new()
        .content("")
        .embeds(embeds)
        .components(vec![
            copy_modal::buttons(&page).await,
            active::buttons(&page).await,
            pagination::buttons(page_index, profiles.len(), false),
        ])
}

pub async fn profile_page(
    page_index: usize,
    bot: &Bot,
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<(serenity::all::Message, usize), serenity::Error> {
    command
        .edit_response(&ctx.http, contents(page_index, bot, command).await)
        .await
        .map(|message| (message, page_index))
}
