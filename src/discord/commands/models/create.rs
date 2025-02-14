use serenity::builder::CreateCommand;

pub fn create() -> CreateCommand {
    CreateCommand::new("models").description("display the availible models")
}
