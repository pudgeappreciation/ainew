use serenity::all::{
    CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage,
    EditInteractionResponse, MessageBuilder, MessageId,
};
use serenity::prelude::*;

pub async fn success(ctx: &Context, command: &CommandInteraction) {
    let message = EditInteractionResponse::new().content("Your request has been queued");
    if let Err(why) = command.edit_response(&ctx.http, message).await {
        println!("Cannot respond to slash command: {why}");
    }
}

pub async fn failure(ctx: &Context, command: &CommandInteraction) {
    let content = MessageBuilder::new()
        .push_safe("Failed to queue request :(")
        .build();

    let result = match command.get_response(&ctx.http).await {
        Ok(_) => {
            let initial_response = EditInteractionResponse::new().content(content);
            command
                .edit_response(&ctx.http, initial_response)
                .await
                .map(|_| ())
        }
        Err(_) => {
            let message = CreateInteractionResponseMessage::new().content(content);
            let initial_response = CreateInteractionResponse::Message(message);
            command.create_response(&ctx.http, initial_response).await
        }
    };

    if let Err(why) = result {
        println!("Cannot respond to slash command: {why}");
    }
}

pub async fn init(
    ctx: &Context,
    command: &CommandInteraction,
) -> Result<MessageId, serenity::Error> {
    let message = CreateInteractionResponseMessage::new().content("Queuing request...");
    let initial_response = CreateInteractionResponse::Defer(message);
    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");
    }

    command
        .get_response(&ctx.http)
        .await
        .map(|message| message.id)
}
