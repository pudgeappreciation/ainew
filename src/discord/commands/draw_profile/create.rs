use serenity::builder::CreateCommand;

use super::new;

pub fn create() -> CreateCommand {
    CreateCommand::new("profile")
        .description("Manage your draw settings profiles")
        .add_option(new::create())
}
