use serenity::builder::CreateCommand;

pub fn create() -> CreateCommand {
    CreateCommand::new("loras").description("display the availible loras")
}
