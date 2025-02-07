pub mod draw;
pub mod draw_queue;

use std::time::Duration;

use draw::draw;
use draw_queue::get_next_request;
use sqlx::{Pool, Sqlite};
use tokio::time::sleep;

use crate::global::{
    channels::{respond_to_message::RespondToMessage, wake_drawer::WakeDrawerReceiver},
    responses::Response,
};

pub async fn draw_session(database: &Pool<Sqlite>, responder: &RespondToMessage) {
    while let Some(request) = get_next_request(database).await {
        if request.drawing(database).await.is_err() {
            continue;
        }

        responder.respond(Response::StartingDrawing {
            channel_id: request.channel_id,
            message_id: request.request_id,
            user_id: request.user_id,
        });
        let result = draw(&request).await;
        match result {
            Ok(response) => {
                responder.respond(Response::DrawingResponse {
                    response,
                    channel_id: request.channel_id,
                    message_id: request.request_id,
                });
            }
            Err(_) => {
                println!("drawing failed");
            }
        }

        _ = request.complete(database).await;
        sleep(Duration::from_secs(1)).await;
    }
}

pub fn start_drawer(
    database: Pool<Sqlite>,
    mut receiver: WakeDrawerReceiver,
    responder: RespondToMessage,
) {
    tokio::spawn(async move {
        loop {
            receiver.wake().await;
            draw_session(&database, &responder).await;
        }
    });
}
