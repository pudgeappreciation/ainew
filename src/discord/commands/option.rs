#![allow(dead_code)]
use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedOption, ResolvedValue};

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

pub fn get_int<'a, T>(name: impl Into<String>, options: T) -> Option<i64>
where
    T: Iterator<Item = &'a ResolvedOption<'a>>,
{
    let name = name.into();

    for option in options {
        if let ResolvedOption {
            name: option_name,
            value: ResolvedValue::Integer(value),
            ..
        } = option
        {
            if option_name == &&name {
                return Some(*value);
            }
        };
    }

    None
}

pub fn get_string<'a, T>(name: impl Into<String>, options: T) -> Option<&'a str>
where
    T: Iterator<Item = &'a ResolvedOption<'a>>,
{
    let name = name.into();

    for option in options {
        if let ResolvedOption {
            name: option_name,
            value: ResolvedValue::String(value),
            ..
        } = option
        {
            if option_name == &&name {
                return Some(value);
            }
        };
    }

    None
}

pub fn get_number<'a, T>(name: impl Into<String>, options: T) -> Option<f64>
where
    T: Iterator<Item = &'a ResolvedOption<'a>>,
{
    let name = name.into();

    for option in options {
        if let ResolvedOption {
            name: option_name,
            value: ResolvedValue::Number(value),
            ..
        } = option
        {
            if option_name == &&name {
                return Some(*value);
            }
        };
    }

    None
}

pub fn get_bool<'a, T>(name: impl Into<String>, options: T) -> Option<bool>
where
    T: Iterator<Item = &'a ResolvedOption<'a>>,
{
    let name = name.into();

    for option in options {
        if let ResolvedOption {
            name: option_name,
            value: ResolvedValue::Boolean(value),
            ..
        } = option
        {
            if option_name == &&name {
                return Some(*value);
            }
        };
    }

    None
}
