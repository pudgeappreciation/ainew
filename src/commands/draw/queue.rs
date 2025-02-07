use serenity::all::{
    CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditInteractionResponse, MessageId,
};
use serenity::prelude::*;
use sqlx::{Pool, Sqlite};

use crate::draw_request::DrawRequest;

async fn send_success_response(ctx: &Context, command: &CommandInteraction) {
    let message = EditInteractionResponse::new().content("Your request has been queued");
    if let Err(why) = command.edit_response(&ctx.http, message).await {
        println!("Cannot respond to slash command: {why}");
    }
}

async fn send_failure_response(ctx: &Context, command: &CommandInteraction) {
    let message = CreateInteractionResponseMessage::new().content("Failed to queue request :(");
    let initial_response = CreateInteractionResponse::Message(message);
    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");
    }
}

async fn init_response(
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<MessageId, serenity::Error> {
    let message = CreateInteractionResponseMessage::new().content("Queuing request...");
    let initial_response = CreateInteractionResponse::Message(message);
    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");
    }

    command
        .get_response(&ctx.http)
        .await
        .map(|message| message.id)
}

pub async fn queue<'a>(database: &Pool<Sqlite>, ctx: Context, command: CommandInteraction) {
    let Some(request_id) = init_response(&ctx, &command).await.ok() else {
        send_failure_response(&ctx, &command).await;
        return;
    };

    let request = DrawRequest::create(&command, request_id);
    let result = request.save(database);

    match result.await {
        Ok(_) => send_success_response(&ctx, &command).await,
        Err(_) => send_failure_response(&ctx, &command).await,
    }
}
