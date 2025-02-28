use serenity::builder::CreateCommand;

pub fn create() -> CreateCommand {
    CreateCommand::new("image_info").kind(serenity::all::CommandType::Message)
}
