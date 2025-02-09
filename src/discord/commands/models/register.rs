use serenity::all::{Command, Context, GuildId};
use serenity::builder::CreateCommand;

fn create_command() -> CreateCommand {
    CreateCommand::new("models").description("display the availible models")
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
