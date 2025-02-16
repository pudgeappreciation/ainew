use serenity::builder::CreateCommand;

use crate::discord::commands::option;

pub fn create() -> CreateCommand {
    CreateCommand::new("draw")
        .description("draw an image")
        .add_option(option::string("prompt", "The prompt to use").required(true))
        .add_option(option::string(
            "negative_prompt",
            "The negative_prompt to use",
        ))
        .add_option(
            option::string(
                "size",
                "The size to use, formatted as {width}x{height}, in pixels",
            )
            .set_autocomplete(true),
        )
        .add_option(option::string("model", "The model to use").set_autocomplete(true))
        .add_option(option::number("cfg_scale", "The cfg scale to use"))
        .add_option(option::int("steps", "The steps to use, default 10"))
        .add_option(option::string("sampler", "The sampler to use").set_autocomplete(true))
        .add_option(
            option::string(
                "scheduler",
                "The scheduler to use (Leaving this blank is usually best)",
            )
            .set_autocomplete(true),
        )
        .add_option(option::int("clip_skip", "The clip skip to use"))
        .add_option(option::int("seed", "The seed to use"))
}
