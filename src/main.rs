mod database;
mod discord;
mod draw_task;
mod global;

use std::sync::Arc;

use database::get_database;
use discord::bot::Bot;
use global::channels::respond_to_message::{self};
use global::channels::wake_draw_task::{self};

use global::generation_options::{base_model, lora};
use serenity::prelude::*;
use tokio;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let discord_token =
        dotenvy::var("DISCORD_TOKEN").expect("Expected a token for Discord in the environment");
    let intents = GatewayIntents::non_privileged();

    let database = get_database().await;
    let (sender, response_receiver) = respond_to_message::make();
    let (draw_task_sender, draw_task_receiver) = wake_draw_task::make();
    draw_task::start(database.clone(), draw_task_receiver, sender);

    let models = Arc::new(RwLock::new(base_model::get().await));
    let loras = Arc::new(RwLock::new(lora::get().await));

    let mut client = Client::builder(discord_token, intents)
        .event_handler(Bot::new(
            database.clone(),
            draw_task_sender.clone(),
            response_receiver,
            models.clone(),
            loras.clone(),
        ))
        .await
        .expect("Failed to create Serenity client");

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
