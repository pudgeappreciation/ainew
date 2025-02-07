mod discord;
mod draw_task;
mod global;
mod responder;

use std::sync::atomic::{AtomicBool, Ordering};

use global::channels::respond_to_message::{self, RespondToMessageReceiver};
use global::channels::wake_draw_task::{self, WakeDrawTask};
use responder::start_responder;
use sqlx::{Pool, Sqlite};
use tokio;

use serenity::async_trait;
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
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

pub struct Bot {
    pub database: Pool<Sqlite>,
    pub draw_task: WakeDrawTask,
    response_receiver: RespondToMessageReceiver,
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            match command.data.name.as_str() {
                "draw" => {
                    discord::commands::draw::queue(&self.database, ctx, command).await;
                    self.draw_task.wake();
                }
                _ => println!("Command not registered"),
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        if let Ok(guild_id) = dotenvy::var("TEST_GUILD_ID") {
            let guild_id = GuildId::new(guild_id.parse().expect("Expected a valid test guild ID"));

            let commands = guild_id
                .set_commands(&ctx.http, vec![discord::commands::draw::register()])
                .await;

            println!("I now have the following test guild slash commands: {commands:#?}");
        }

        if dotenvy::var("APP_ENV").unwrap_or("dev".to_string()) == "production" {
            let guild_command =
                Command::create_global_command(&ctx.http, discord::commands::draw::register())
                    .await;

            println!("I created the following global slash command: {guild_command:#?}");
        }
    }

    // We use the cache_ready event just in case some cache operation is required in whatever use
    // case you have for this.
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");

        if !self.is_loop_running.load(Ordering::Relaxed) {
            start_responder(ctx.clone(), self.response_receiver.clone());

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }
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
        .event_handler(Bot {
            database: database.clone(),
            draw_task: draw_task_sender.clone(),
            response_receiver,
            is_loop_running: AtomicBool::new(false),
        })
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
