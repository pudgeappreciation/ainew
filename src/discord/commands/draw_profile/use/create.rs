use serenity::all::{CommandOptionType, CreateCommandOption};

use crate::discord::commands::option;

pub fn create() -> CreateCommandOption {
    CreateCommandOption::new(
        CommandOptionType::SubCommand,
        "use",
        "Set one of your profiles as active",
    )
    .add_sub_option(
        option::string(
            "profile_name",
            "The name of the profile to set as active, ignore to clear your active profile",
        )
        .set_autocomplete(true),
    )
}
