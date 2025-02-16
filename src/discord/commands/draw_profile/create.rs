use serenity::builder::CreateCommand;

use super::{delete, new, r#use};

pub fn create() -> CreateCommand {
    CreateCommand::new("profile")
        .description("Manage your draw settings profiles")
        .add_option(delete::create())
        .add_option(new::create())
        .add_option(r#use::create())
}
