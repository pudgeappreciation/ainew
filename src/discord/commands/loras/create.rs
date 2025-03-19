use serenity::builder::CreateCommand;

use super::{compact_list, list_with_images};

pub fn create() -> CreateCommand {
    CreateCommand::new("loras")
        .description("Manage your draw settings profiles")
        .add_option(list_with_images::create())
        .add_option(compact_list::create())
}
