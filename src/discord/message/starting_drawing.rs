use serenity::all::{CreateMessage, MessageBuilder, UserId};

pub fn create(user_id: UserId) -> CreateMessage {
    let content = MessageBuilder::new()
        .push("Starting work on your drawing!")
        .mention(&user_id)
        .build();

    CreateMessage::new().content(content)
}
