pub mod draw;
pub mod draw_queue;

use std::time::Duration;

use draw::draw;
use draw_queue::get_next_request;
use sqlx::{Pool, Sqlite};
use tokio::time::sleep;

use crate::{
    discord::message,
    global::channels::{
        respond_to_message::RespondToMessage, wake_draw_task::WakeDrawTaskReceiver,
    },
};

async fn draw_session(database: &Pool<Sqlite>, responder: &RespondToMessage) {
    while let Some(request) = get_next_request(database).await {
        if request.drawing(database).await.is_err() {
            continue;
        }

        responder.send(
            message::starting_drawing(request.user_id),
            request.channel_id,
        );
        let result = draw(&request).await;
        match result {
            Ok(response) => {
                responder.send(
                    message::finished_drawing(response, request.channel_id, request.message_id),
                    request.channel_id,
                );
                _ = request.complete(database).await;
            }
            Err(_) => {
                println!("drawing failed");
                _ = request.failed(database).await;
            }
        }

        sleep(Duration::from_secs(1)).await;
    }
}

pub fn start(
    database: Pool<Sqlite>,
    mut receiver: WakeDrawTaskReceiver,
    responder: RespondToMessage,
) {
    tokio::spawn(async move {
        loop {
            receiver.wake().await;
            draw_session(&database, &responder).await;
        }
    });
}
