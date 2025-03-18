use serenity::all::{CommandOptionType, CreateCommandOption};

use crate::discord::commands::option;

pub fn create() -> CreateCommandOption {
    let description = "Display the available loras with preview images";

    CreateCommandOption::new(CommandOptionType::SubCommand, "images", description)
        .add_sub_option(option::int("page", "Jump to a specific page").min_int_value(1))
}
