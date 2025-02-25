use serenity::builder::CreateCommand;

use super::{delete, list, new, r#use, reset};

pub fn create() -> CreateCommand {
    CreateCommand::new("profile")
        .description("Manage your draw settings profiles")
        .add_option(delete::create())
        .add_option(list::create())
        .add_option(new::create())
        .add_option(reset::create())
        .add_option(r#use::create())
}
