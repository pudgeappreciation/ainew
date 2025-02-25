use serenity::builder::CreateCommand;

use super::{delete, list, r#use, reset, set};

pub fn create() -> CreateCommand {
    CreateCommand::new("profile")
        .description("Manage your draw settings profiles")
        .add_option(delete::create())
        .add_option(list::create())
        .add_option(set::create())
        .add_option(reset::create())
        .add_option(r#use::create())
}
