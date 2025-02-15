use serenity::builder::CreateCommand;

use crate::discord::commands::option;

pub fn create() -> CreateCommand {
    CreateCommand::new("loras")
        .description("display the availible loras")
        .add_option(
            option::int("page", "Jump to a specific page")
                .required(false)
                .min_int_value(1),
        )
}
