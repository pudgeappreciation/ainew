use serde::{Deserialize, Serialize};
use serenity::all::{MessageBuilder, ResolvedOption, ResolvedValue};

use crate::discord::{
    commands::utilities::push_command_option::AddCommandOption,
    message::body::name_value_pair::AddOptionalNameValuePair,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Options {
    pub prompt_head: Option<String>,
    pub prompt_tail: Option<String>,
    pub negative_prompt_head: Option<String>,
    pub negative_prompt_tail: Option<String>,
    pub sampler: Option<String>,
    pub scheduler: Option<String>,
    pub model: Option<String>,
    pub vae: Option<String>,
    pub steps: Option<u8>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub clip_skip: Option<u8>,
    pub cfg_scale: Option<f32>,
}

impl Options {
    pub fn new_from_command<'a>(command_options: &Vec<ResolvedOption<'a>>) -> Options {
        let mut options = Self::default();

        for option in command_options.iter() {
            match option {
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "prompt_head",
                    ..
                } => options.prompt_head = Some(value.to_string()),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "prompt_tail",
                    ..
                } => options.prompt_tail = Some(value.to_string()),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "negative_prompt_head",
                    ..
                } => options.negative_prompt_head = Some(value.to_string()),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "negative_prompt_tail",
                    ..
                } => options.negative_prompt_tail = Some(value.to_string()),
                ResolvedOption {
                    value: ResolvedValue::Integer(value),
                    name: "steps",
                    ..
                } => options.steps = Some(*value as u8),
                ResolvedOption {
                    value: ResolvedValue::Integer(value),
                    name: "clip_skip",
                    ..
                } => options.clip_skip = Some(*value as u8),
                ResolvedOption {
                    value: ResolvedValue::Number(value),
                    name: "cfg_scale",
                    ..
                } => options.cfg_scale = Some(*value as f32),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "sampler",
                    ..
                } => options.sampler = Some(value.to_string()),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "scheduler",
                    ..
                } => options.scheduler = Some(value.to_string()),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "model",
                    ..
                } => options.model = Some(value.to_string()),
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
                            options.width = Some(width);
                            options.height = Some(height);
                        }
                        _ => (),
                    }
                }
                _ => {}
            }
        }

        options
    }

    pub fn embed(&self, content: &mut MessageBuilder) {
        content
            .append_name_value_pair("Prompt Head: ", &self.prompt_head)
            .append_name_value_pair("Prompt Tail: ", &self.prompt_tail)
            .append_name_value_pair("Negative Prompt Head: ", &self.negative_prompt_head)
            .append_name_value_pair("Negative Prompt Tail: ", &self.negative_prompt_tail)
            .append_name_value_pair("Sampler: ", &self.sampler)
            .append_name_value_pair("Scheduler: ", &self.scheduler)
            .append_name_value_pair("Model: ", &self.model)
            .append_name_value_pair("Vae: ", &self.vae)
            .append_name_value_pair("Steps: ", &self.steps)
            .append_name_value_pair("Width: ", &self.width)
            .append_name_value_pair("Height: ", &self.height)
            .append_name_value_pair("Clip Skip: ", &self.clip_skip)
            .append_name_value_pair("Cfg Scale: ", &self.cfg_scale);
    }

    pub fn to_command_options(&self) -> String {
        let size = match (&self.width, &self.height) {
            (Some(width), Some(height)) => Some(format!("size:{}x{}", width, height)),
            _ => None,
        };

        let mut command = String::new();
        command
            .append_command_option("prompt_head", &self.prompt_head)
            .append_command_option("prompt_tail", &self.prompt_head)
            .append_command_option("negative_prompt_head", &self.prompt_head)
            .append_command_option("negative_prompt_tail", &self.prompt_head)
            .append_command_option("sampler", &self.sampler)
            .append_command_option("scheduler", &self.scheduler)
            .append_command_option("model", &self.model)
            .append_command_option("vae", &self.vae)
            .append_command_option("steps", &self.steps)
            .append_command_option("size", &size)
            .append_command_option("clip_skip", &self.clip_skip)
            .append_command_option("cfg_scale", &self.cfg_scale);

        command
    }

    pub fn merge(mut self, other: &Self) -> Self {
        self.prompt_head = self.prompt_head.or(other.prompt_head.clone());
        self.prompt_tail = self.prompt_tail.or(other.prompt_tail.clone());
        self.negative_prompt_head = self
            .negative_prompt_head
            .or(other.negative_prompt_head.clone());
        self.negative_prompt_tail = self
            .negative_prompt_tail
            .or(other.negative_prompt_tail.clone());
        self.sampler = self.sampler.or(other.sampler.clone());
        self.scheduler = self.scheduler.or(other.scheduler.clone());
        self.model = self.model.or(other.model.clone());
        self.vae = self.vae.or(other.vae.clone());
        self.steps = self.steps.or(other.steps);
        self.width = self.width.or(other.width);
        self.height = self.height.or(other.height);
        self.clip_skip = self.clip_skip.or(other.clip_skip);
        self.cfg_scale = self.cfg_scale.or(other.cfg_scale);

        return self;
    }
}
