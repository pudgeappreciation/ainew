use serenity::all::CommandInteraction;
use serenity::prelude::*;
use sqlx::{Pool, Sqlite};

use crate::global::draw_request::DrawRequest;

use super::respond;

pub async fn queue<'a>(database: &Pool<Sqlite>, ctx: Context, command: CommandInteraction) {
    let Some(message_id) = respond::init(&ctx, &command).await.ok() else {
        respond::failure(&ctx, &command).await;
        return;
    };

    let request = DrawRequest::new_from_command(&command, message_id);
    let result = request.save(database);

    match result.await {
        Ok(_) => respond::success(&ctx, &command).await,
        Err(_) => respond::failure(&ctx, &command).await,
    }
}
