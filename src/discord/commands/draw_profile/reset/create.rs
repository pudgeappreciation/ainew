use serenity::all::{CommandOptionType, CreateCommandOption};

use crate::discord::commands::option;

pub fn create() -> CreateCommandOption {
    let description = "Reset some of the fields in one of your profiles";

    CreateCommandOption::new(CommandOptionType::SubCommand, "reset", description)
        .add_sub_option(
            option::bool("profile_name", "The name of the profile to reset fields in")
                .required(true)
                .set_autocomplete(true),
        )
        .add_sub_option(option::bool(
            "prompt_head",
            "The tokens to prepend to the prompt",
        ))
        .add_sub_option(option::bool(
            "prompt_tail",
            "The tokens to append to the prompt",
        ))
        .add_sub_option(option::bool(
            "negative_prompt_head",
            "The tokens to prepend to the negative prompt",
        ))
        .add_sub_option(option::bool(
            "negative_prompt_tail",
            "The tokens to append to the negative prompt",
        ))
        .add_sub_option(option::bool("size", "The size to use"))
        .add_sub_option(option::bool("model", "The model to use"))
        .add_sub_option(option::bool("cfg_scale", "The cfg scale to use"))
        .add_sub_option(option::bool("steps", "The steps to use, default 10"))
        .add_sub_option(option::bool("sampler", "The sampler to use"))
        .add_sub_option(option::bool("scheduler", "The scheduler to use"))
        .add_sub_option(option::bool("clip_skip", "The clip skip to use"))
}
