use rand::RngCore;
use serde::{Deserialize, Serialize};
use serenity::all::{CommandInteraction, ResolvedOption, ResolvedValue};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    pub prompt: String,
    pub negative_prompt: String,
    pub sampler: String,
    pub scheduler: String,
    pub model: String,
    pub vae: Option<String>,
    pub steps: u8,
    pub seed: u32,
    pub width: u32,
    pub height: u32,
    pub clip_skip: u8,
    pub cfg_scale: f32,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            prompt: String::from(""),
            negative_prompt: String::from(""),
            sampler: String::from("Euler a"),
            scheduler: String::from("Automatic"),
            model: String::from(""),
            vae: None,
            steps: 20,
            seed: rand::rng().next_u32(),
            width: 512,
            height: 512,
            clip_skip: 0,
            cfg_scale: 7.5,
        }
    }
}

impl Options {
    pub fn new_from_command(command: &CommandInteraction) -> Options {
        let mut options = Options::default();

        for option in command.data.options().iter() {
            match option {
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "prompt",
                    ..
                } => options.prompt = value.to_string(),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "negative_prompt",
                    ..
                } => options.negative_prompt = value.to_string(),
                ResolvedOption {
                    value: ResolvedValue::Integer(value),
                    name: "steps",
                    ..
                } => options.steps = *value as u8,
                ResolvedOption {
                    value: ResolvedValue::Integer(value),
                    name: "seed",
                    ..
                } => options.seed = *value as u32,
                ResolvedOption {
                    value: ResolvedValue::Integer(value),
                    name: "clip_skip",
                    ..
                } => options.clip_skip = *value as u8,
                ResolvedOption {
                    value: ResolvedValue::Number(value),
                    name: "cfg_scale",
                    ..
                } => options.cfg_scale = *value as f32,
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "sampler",
                    ..
                } => options.sampler = value.to_string(),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "scheduler",
                    ..
                } => options.scheduler = value.to_string(),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "model",
                    ..
                } => options.model = value.to_string(),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "size",
                    ..
                } => {
                    let mut dimensions = value
                        .split(['x', 'X'])
                        .map(|dimension| dimension.parse::<u32>().ok());
                    match (dimensions.next(), dimensions.next()) {
                        (Some(Some(width)), Some(Some(height))) => {
                            options.width = width;
                            options.height = height;
                        }
                        _ => (),
                    }
                }
                _ => {}
            }
        }

        options
    }
}
