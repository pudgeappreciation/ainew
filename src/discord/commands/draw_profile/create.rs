use serenity::builder::CreateCommand;

pub fn create() -> CreateCommand {
    CreateCommand::new("profile").description("Manage your draw settings profiles")
}
