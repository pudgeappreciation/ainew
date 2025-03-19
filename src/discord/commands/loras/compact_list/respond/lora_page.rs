use serenity::all::{CommandInteraction, Context, EditInteractionResponse, MessageBuilder};

use crate::{
    discord::{bot::Bot, commands::utilities::pagination},
    global::generation_options::lora::Lora,
};

fn empty_response() -> EditInteractionResponse {
    EditInteractionResponse::new()
        .content("No loras registered")
        .embeds(Vec::new())
        .components(Vec::new())
        .clear_attachments()
}

fn page_content<T, U, L>(superpage_index: usize, loras: T) -> MessageBuilder
where
    T: IntoIterator<Item = U>,
    U: IntoIterator<Item = L>,
    L: Into<Lora>,
{
    let mut content = MessageBuilder::new();
    content.push_bold_line_safe("Loras:");

    for (subpage_index, chunk) in loras.into_iter().enumerate() {
        let page = subpage_index + (superpage_index * 5) + 1;
        content.push_line_safe("");
        content.push_bold_line_safe(format!("Page {}:", page));

        for lora in chunk.into_iter() {
            let lora = lora.into();
            let internal_name = lora.internal_name.clone();
            content
                .push_safe(format!(
                    "{}: ",
                    lora.civitai_info
                        .as_ref()
                        .map(|info| &info.name)
                        .unwrap_or(&internal_name),
                ))
                .push_mono_line_safe(format!("<lora:{}:1.0>", internal_name));
        }
    }

    println!("{}", content.build());

    content
}

async fn contents(page_index: usize, bot: &Bot) -> EditInteractionResponse {
    let loras = bot.loras.read().await;
    let Some(page) = pagination::page(loras.chunks(5), page_index) else {
        return empty_response();
    };

    EditInteractionResponse::new()
        .content(page_content(page_index, page).build())
        .components(vec![pagination::buttons(
            page_index,
            loras.len().div_ceil(5),
            false,
        )])
}

pub async fn lora_page(
    page_index: usize,
    bot: &Bot,
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<(serenity::all::Message, usize), serenity::Error> {
    command
        .edit_response(&ctx.http, contents(page_index, bot).await)
        .await
        .map(|message| (message, page_index))
}
