pub mod draw;
pub mod draw_queue;

use std::time::Duration;

use draw::draw;
use draw_queue::get_next_request;
use sqlx::{Pool, Sqlite};
use tokio::{
    sync::watch::{self, Sender},
    time::sleep,
};

use crate::responder::{Response, ResponseSender};

pub async fn draw_session(database: &Pool<Sqlite>, responder: &ResponseSender) {
    while let Some(request) = get_next_request(database).await {
        if request.drawing(database).await.is_err() {
            continue;
        }

        responder.send(Response::StartingDrawing {
            channel_id: request.channel_id,
            message_id: request.request_id,
            user_id: request.user_id,
        });
        let result = draw(&request).await;
        match result {
            Ok(response) => {
                responder.send(Response::DrawingResponse {
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

struct DrawerPing;

#[derive(Clone)]
pub struct PingDrawer(Sender<DrawerPing>);

impl PingDrawer {
    pub fn ping(&self) {
        _ = self.0.send(DrawerPing);
    }
}

pub fn start_drawer(database: Pool<Sqlite>, responder: ResponseSender) -> PingDrawer {
    let (tx, mut watcher) = watch::channel(DrawerPing);

    tokio::spawn(async move {
        loop {
            watcher.borrow_and_update();
            _ = watcher.changed().await;
            draw_session(&database, &responder).await;
        }
    });

    PingDrawer(tx)
}
