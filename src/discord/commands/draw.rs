mod queue;

use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;

pub use queue::queue;

pub fn register() -> CreateCommand {
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
