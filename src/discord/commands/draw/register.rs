use serenity::all::{Command, Context, GuildId};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

fn create_command() -> CreateCommand {
    CreateCommand::new("draw")
        .description("draw an image")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "prompt", "The prompt to use")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "negative_prompt",
                "The negative_prompt to use",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "steps",
                "The steps to use, default 10",
            )
            .required(false),
        )
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
