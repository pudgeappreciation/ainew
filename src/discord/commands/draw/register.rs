use serenity::all::{Command, Context, GuildId};
use serenity::builder::CreateCommand;

use crate::discord::commands::option;

fn create_command() -> CreateCommand {
    CreateCommand::new("draw")
        .description("draw an image")
        .add_option(option::string("prompt", "The prompt to use").required(true))
        .add_option(option::string("negative_prompt", "The negative_prompt to use").required(false))
        .add_option(
            option::string(
                "size",
                "The size to use, formatted as {width}x{height}, in pixels",
            )
            .required(false),
        )
        .add_option(option::string("model", "The model to use").required(false))
        .add_option(option::number("cfg_scale", "The cfg scale to use").required(false))
        .add_option(option::int("steps", "The steps to use, default 10").required(false))
        .add_option(option::string("sampler", "The sampler to use").required(false))
        .add_option(option::string("scheduler", "The scheduler to use").required(false))
        .add_option(option::int("clip_skip", "The clip skip to use").required(false))
        .add_option(option::int("seed", "The seed to use").required(false))
}

pub async fn register(ctx: &Context) {
    if let Ok(guild_id) = dotenvy::var("TEST_GUILD_ID") {
        let guild_id = GuildId::new(guild_id.parse().expect("Expected a valid test guild ID"));

        let commands = guild_id
            .set_commands(&ctx.http, vec![create_command()])
            .await;

        println!("I now have the following test guild slash commands: {commands:#?}");
    }

    if dotenvy::var("APP_ENV").unwrap_or("dev".to_string()) == "production" {
        let guild_command = Command::create_global_command(&ctx.http, create_command()).await;

        println!("I created the following global slash command: {guild_command:#?}");
    }
}
