use serenity::all::{CommandOptionType, CreateCommandOption};

use crate::discord::commands::option;

pub fn create() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "delete",
        "Delete one of your profiles",
    )
    .add_sub_option(
        option::string("profile_name", "The name of the pofile to delete")
            .required(true)
            .set_autocomplete(true),
    )
}
