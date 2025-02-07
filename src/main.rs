mod commands;

use serenity::all::{
    CreateAttachment, CreateInteractionResponse, CreateInteractionResponseMessage, EditAttachments,
    EditInteractionResponse,
};
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
            println!("Received command interaction: {command:#?}");

            let options = command.data.options();

            let content = match command.data.name.as_str() {
                "draw" => Some(commands::draw::run(&options)),
                _ => None,
            };

            if let Some(content) = content {
                let initial_message =
                    CreateInteractionResponseMessage::new().content("**Generating image...**");
                let initial_response = CreateInteractionResponse::Message(initial_message);
                if let Err(why) = command.create_response(&ctx.http, initial_response).await {
                    println!("Cannot respond to slash command: {why}");
                }

                let (content, image) = content.await;
                let mut edit = EditInteractionResponse::new().content(content);
                if let Some(image) = image {
                    let edit_attachments =
                        EditAttachments::new().add(CreateAttachment::bytes(image, "image.png"));
                    edit = edit.attachments(edit_attachments);
                }
                if let Err(why) = command.edit_response(&ctx.http, edit).await {
                    println!("Cannot edit response: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        if let Ok(guild_id) = dotenv::var("TEST_GUILD_ID") {
            let guild_id = GuildId::new(guild_id.parse().expect("Expected a valid test guild ID"));

            let commands = guild_id
                .set_commands(&ctx.http, vec![commands::draw::register()])
                .await;

            println!("I now have the following test guild slash commands: {commands:#?}");
        }

        if dotenv::var("APP_ENV").unwrap_or("dev".to_string()) == "production" {
            let guild_command =
                Command::create_global_command(&ctx.http, commands::draw::register()).await;

            println!("I created the following global slash command: {guild_command:#?}");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let discord_token =
        dotenv::var("DISCORD_TOKEN").expect("Expected a token for Discord in the environment");
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
