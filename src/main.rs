mod commands;

use tokio;

use serenity::async_trait;
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:?}");

            match command.data.name.as_str() {
                "draw" => _ = commands::draw::run(ctx, command).await,
                _ => println!("Command not registered"),
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        if let Ok(guild_id) = dotenvy::var("TEST_GUILD_ID") {
            let guild_id = GuildId::new(guild_id.parse().expect("Expected a valid test guild ID"));

            let commands = guild_id
                .set_commands(&ctx.http, vec![commands::draw::register()])
                .await;

            println!("I now have the following test guild slash commands: {commands:#?}");
        }

        if dotenvy::var("APP_ENV").unwrap_or("dev".to_string()) == "production" {
            let guild_command =
                Command::create_global_command(&ctx.http, commands::draw::register()).await;

            println!("I created the following global slash command: {guild_command:#?}");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let discord_token =
        dotenvy::var("DISCORD_TOKEN").expect("Expected a token for Discord in the environment");
    let intents = GatewayIntents::non_privileged();

    let mut client = Client::builder(discord_token, intents)
        .event_handler(Handler)
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
