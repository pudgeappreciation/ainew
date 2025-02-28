use rand::RngCore;
use serde::{Deserialize, Serialize};
use serenity::all::{CommandInteraction, MessageBuilder, ResolvedOption, ResolvedValue};

use crate::{
    discord::{
        commands::utilities::push_command_option::AddCommandOption,
        message::body::{
            name_value_pair::AddNameValuePair, optional_name_value_pair::AddOptionalNameValuePair,
        },
    },
    global::draw_profile::DrawProfile,
};

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

impl From<&DrawProfile> for Options {
    fn from(value: &DrawProfile) -> Self {
        Self {
            sampler: value.options.sampler.clone().unwrap_or("Euler a".into()),
            scheduler: value
                .options
                .scheduler
                .clone()
                .unwrap_or("Automatic".into()),
            model: value.options.model.clone().unwrap_or_default(),
            vae: value.options.vae.clone(),
            steps: value.options.steps.unwrap_or(20),
            width: value.options.width.unwrap_or(512),
            height: value.options.height.unwrap_or(512),
            clip_skip: value.options.clip_skip.unwrap_or(0),
            cfg_scale: value.options.cfg_scale.unwrap_or(7.5),
            ..Default::default()
        }
    }
}

impl Options {
    pub fn new_from_command(command: &CommandInteraction, profile: Option<DrawProfile>) -> Options {
        let mut options: Options = profile
            .as_ref()
            .map(|profile| profile.into())
            .unwrap_or_default();

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

        if let Some(profile) = profile.as_ref() {
            options.prompt = profile.wrap_prompt(options.prompt);
            options.negative_prompt = profile.wrap_negative_prompt(options.negative_prompt);
        };

        options
    }

    pub fn embed(&self, content: &mut MessageBuilder) {
        content
            .push_bold_safe("Prompt: ")
            .push_codeblock_safe(&self.prompt, None)
            .push_bold_safe("Negative Prompt: ");

        if self.negative_prompt.is_empty() {
            content.push_line_safe("None");
        } else {
            content.push_codeblock_safe(&self.negative_prompt, None);
        }

        if self.model.is_empty() {
            content.append_name_value("Model: ", &"N/A");
        } else {
            content.append_name_value("Model: ", &self.model);
        }

        content
            .append_name_value("Sampler: ", &self.sampler)
            .append_name_value("Scheduler: ", &self.scheduler)
            .append_optional_name_value("Vae: ", &self.vae)
            .append_name_value("Steps: ", &self.steps)
            .append_name_value("Width: ", &self.width)
            .append_name_value("Height: ", &self.height)
            .append_name_value("Clip Skip: ", &self.clip_skip)
            .append_name_value("Cfg Scale: ", &self.cfg_scale)
            .append_name_value("Seed: ", &self.seed);
    }

    pub fn to_command_options(&self) -> String {
        let mut command = String::new();
        command.append_command_option("prompt", &Some(&self.prompt));

        if !self.negative_prompt.is_empty() {
            command.append_command_option("negative_prompt", &Some(&self.negative_prompt));
        }

        command.append_command_option(
            "size",
            &Some(format!("size:{}x{}", &self.width, &self.height)),
        );

        if !self.model.is_empty() {
            command.append_command_option("model", &Some(&self.model));
        }

        command
            .append_command_option("cfg_scale", &Some(&self.cfg_scale))
            .append_command_option("steps", &Some(&self.steps))
            .append_command_option("sampler", &Some(&self.sampler))
            .append_command_option("scheduler", &Some(&self.scheduler))
            .append_command_option("clip_skip", &Some(&self.clip_skip))
            .append_command_option("seed", &Some(&self.seed));

        command
    }
}
