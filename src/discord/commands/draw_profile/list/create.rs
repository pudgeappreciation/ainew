use serenity::all::{CommandOptionType, CreateCommandOption};

use crate::discord::commands::option;

pub fn create() -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::SubCommand, "list", "View your profiles")
        .add_sub_option(option::int("page", "Jump to a specific page").min_int_value(1))
}
