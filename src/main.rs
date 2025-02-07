mod discord;
mod draw_task;
mod global;

use discord::bot::Bot;
use global::channels::respond_to_message::{self};
use global::channels::wake_draw_task::{self};
use sqlx::{Pool, Sqlite};
use tokio;

use serenity::prelude::*;

async fn get_database() -> Pool<Sqlite> {
    // Initiate a connection to the database file, creating the file if required.
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("sqlite.db")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    // Run migrations, which updates the database's schema to the latest version.
    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    database
}

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

    let mut client = Client::builder(discord_token, intents)
        .event_handler(Bot::new(
            database.clone(),
            draw_task_sender.clone(),
            response_receiver,
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
