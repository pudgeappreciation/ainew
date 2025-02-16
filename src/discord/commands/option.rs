use serenity::all::{CommandOptionType, CreateCommandOption};

pub fn int(name: impl Into<String>, description: impl Into<String>) -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::Integer, name, description)
}

pub fn string(name: impl Into<String>, description: impl Into<String>) -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::String, name, description)
}

pub fn number(name: impl Into<String>, description: impl Into<String>) -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::Number, name, description)
}

pub fn bool(name: impl Into<String>, description: impl Into<String>) -> CreateCommandOption {
    CreateCommandOption::new(CommandOptionType::Boolean, name, description)
}
