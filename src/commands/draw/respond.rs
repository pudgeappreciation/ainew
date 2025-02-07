use serenity::all::{
    CommandInteraction, CreateAttachment, CreateInteractionResponse,
    CreateInteractionResponseFollowup, CreateInteractionResponseMessage, EditInteractionResponse,
    MessageBuilder,
};
use serenity::prelude::*;

use crate::drawer::draw::DrawResponse;

async fn send_initial_interaction_response(ctx: &Context, command: &CommandInteraction) {
    let initial_message = CreateInteractionResponseMessage::new().content("**Drawing image...**");
    let initial_response = CreateInteractionResponse::Message(initial_message);
    println!("Responding to slash command");
    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");
    }
}

async fn close_initial_interaction(ctx: &Context, command: &CommandInteraction) {
    println!("Closing command response");
    let content = MessageBuilder::new()
        .mention(&command.user.id)
        .push(", your drawing is complete.")
        .build();
    let edit = EditInteractionResponse::new().content(content);
    if let Err(why) = command.edit_response(&ctx.http, edit).await {
        println!("Cannot edit response: {why}");
    }
}

fn final_response_message(draw_response: DrawResponse) -> CreateInteractionResponseFollowup {
    let Some(image_data) = draw_response.images.into_iter().next() else {
        return CreateInteractionResponseFollowup::new().content("**No image returned :/**");
    };

    return CreateInteractionResponseFollowup::new()
        .content("**Your drawing! :D**")
        .add_file(CreateAttachment::bytes(image_data, "image.png"));
}

async fn send_final_response(
    ctx: &Context,
    command: &CommandInteraction,
    draw_response: DrawResponse,
) {
    println!("Sending image");
    let message = final_response_message(draw_response);
    if let Err(why) = command.create_followup(&ctx.http, message).await {
        println!("Cannot create response: {why}");
    }
}

async fn send_failure_response(ctx: &Context, command: &CommandInteraction) {
    let edit = EditInteractionResponse::new().content("Drawing failed :(");
    if let Err(why) = command.edit_response(&ctx.http, edit).await {
        println!("Cannot edit response: {why}");
    }
}
