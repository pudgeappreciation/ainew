use serenity::all::{CommandOptionType, CreateCommandOption};

use crate::discord::commands::option;

pub fn create() -> CreateCommandOption {
    let description = "Create a new profile or update an existing one";

    CreateCommandOption::new(CommandOptionType::SubCommand, "new", description)
        .add_sub_option(
            option::string("profile_name", "The name of the profile")
                .required(true)
                .set_autocomplete(true),
        )
        .add_sub_option(
            option::bool("active", "Set this as your active profile (default false)")
                .required(true),
        )
        .add_sub_option(option::string(
            "prompt_head",
            "The tokens to prepend to the prompt",
        ))
        .add_sub_option(option::string(
            "prompt_tail",
            "The tokens to append to the prompt",
        ))
        .add_sub_option(option::string(
            "negative_prompt_head",
            "The tokens to prepend to the negative prompt",
        ))
        .add_sub_option(option::string(
            "negative_prompt_tail",
            "The tokens to append to the negative prompt",
        ))
        .add_sub_option(
            option::string(
                "size",
                "The size to use, formatted as {width}x{height}, in pixels",
            )
            .set_autocomplete(true),
        )
        .add_sub_option(option::string("model", "The model to use").set_autocomplete(true))
        .add_sub_option(option::number("cfg_scale", "The cfg scale to use"))
        .add_sub_option(option::int("steps", "The steps to use, default 10"))
        .add_sub_option(option::string("sampler", "The sampler to use").set_autocomplete(true))
        .add_sub_option(
            option::string(
                "scheduler",
                "The scheduler to use (Leaving this blank is usually best)",
            )
            .set_autocomplete(true),
        )
        .add_sub_option(option::int("clip_skip", "The clip skip to use"))
}
