use serenity::builder::CreateCommand;

use crate::discord::commands::option;

pub fn create() -> CreateCommand {
    CreateCommand::new("models")
        .description("display the availible models")
        .add_option(
            option::int("page", "Jump to a specific page")
                .required(false)
                .min_int_value(1),
        )
}
