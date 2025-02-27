use serenity::builder::CreateCommand;

use crate::discord::commands::option;

pub fn create() -> CreateCommand {
    CreateCommand::new("draw")
        .description("draw an image")
        .add_option(option::string("prompt", "The prompt to use").required(true))
        .add_option(option::string("negative_prompt", "The negative_prompt to use").required(false))
        .add_option(
            option::string(
                "size",
                "The size to use, formatted as {width}x{height}, in pixels",
            )
            .required(false)
            .set_autocomplete(true),
        )
        .add_option(
            option::string("model", "The model to use")
                .required(false)
                .set_autocomplete(true),
        )
        .add_option(option::number("cfg_scale", "The cfg scale to use").required(false))
        .add_option(option::int("steps", "The steps to use, default 10").required(false))
        .add_option(
            option::string("sampler", "The sampler to use")
                .required(false)
                .set_autocomplete(true),
        )
        .add_option(
            option::string(
                "scheduler",
                "The scheduler to use (Leaving this blank is usually best)",
            )
            .required(false)
            .set_autocomplete(true),
        )
        .add_option(option::int("clip_skip", "The clip skip to use").required(false))
        .add_option(option::int("seed", "The seed to use").required(false))
}
