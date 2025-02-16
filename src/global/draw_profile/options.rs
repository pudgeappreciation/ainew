use serde::{Deserialize, Serialize};
use serenity::all::{MessageBuilder, ResolvedOption, ResolvedValue};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Options {
    pub prompt_head: Option<String>,
    pub prompt_tail: Option<String>,
    pub negative_prompt_head: Option<String>,
    pub negative_prompt_tail: Option<String>,
    pub sampler: String,
    pub scheduler: String,
    pub model: String,
    pub vae: Option<String>,
    pub steps: u8,
    pub width: u32,
    pub height: u32,
    pub clip_skip: u8,
    pub cfg_scale: f32,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            prompt_head: None,
            prompt_tail: None,
            negative_prompt_head: None,
            negative_prompt_tail: None,
            sampler: String::from("Euler a"),
            scheduler: String::from("Automatic"),
            model: String::from(""),
            vae: None,
            steps: 20,
            width: 512,
            height: 512,
            clip_skip: 0,
            cfg_scale: 7.5,
        }
    }
}

impl Options {
    pub fn new_from_command<'a>(command_options: &Vec<ResolvedOption<'a>>) -> Options {
        let mut options = Options::default();

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
                } => options.steps = *value as u8,
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

    pub fn embed(&self, content: &mut MessageBuilder) {
        match &self.prompt_head {
            Some(prompt_head) => content
                .push_bold("Prompt Head: ")
                .push_line_safe("")
                .push_codeblock_safe(prompt_head, None),
            None => content.push_bold("Prompt Head: ").push_line_safe("None"),
        };

        match &self.prompt_tail {
            Some(prompt_tail) => content
                .push_bold("Prompt Tail: ")
                .push_line_safe("")
                .push_codeblock_safe(prompt_tail, None),
            None => content.push_bold("Prompt Tail: ").push_line_safe("None"),
        };

        match &self.negative_prompt_head {
            Some(negative_prompt_head) => content
                .push_bold("Negative_Prompt Head: ")
                .push_line_safe("")
                .push_codeblock_safe(negative_prompt_head, None),
            None => content
                .push_bold("Negative_Prompt Head: ")
                .push_line_safe("None"),
        };

        match &self.negative_prompt_tail {
            Some(negative_prompt_tail) => content
                .push_bold("Negative Prompt Tail: ")
                .push_line_safe("")
                .push_codeblock_safe(negative_prompt_tail, None),
            None => content
                .push_bold("Negative Prompt Tail: ")
                .push_line_safe("None"),
        };

        content
            .push_bold("Sampler: ")
            .push_line_safe(&self.sampler)
            .push_bold("Scheduler: ")
            .push_line_safe(&self.scheduler)
            .push_bold("Model: ")
            .push_line_safe(&self.model)
            .push_bold("Vae: ")
            .push_line_safe(self.vae.clone().unwrap_or(String::from("None")))
            .push_bold("Steps: ")
            .push_line_safe(&self.steps.to_string())
            .push_bold("Width: ")
            .push_line_safe(&self.width.to_string())
            .push_bold("Height: ")
            .push_line_safe(&self.height.to_string())
            .push_bold("Clip Skip: ")
            .push_line_safe(&self.clip_skip.to_string())
            .push_bold("Cfg Scale: ")
            .push_line_safe(&self.cfg_scale.to_string());
    }

    pub fn to_command_options(&self) -> String {
        let mut command = String::new();

        if let Some(prompt_head) = &self.prompt_head {
            command.push_str("\nprompt_head:");
            command.push_str(prompt_head);
        };
        if let Some(prompt_tail) = &self.prompt_tail {
            command.push_str("\nprompt_tail:");
            command.push_str(prompt_tail);
        };
        if let Some(negative_prompt_head) = &self.negative_prompt_head {
            command.push_str("\nnegative_prompt_head:");
            command.push_str(negative_prompt_head);
        };
        if let Some(negative_prompt_tail) = &self.negative_prompt_tail {
            command.push_str("\nnegative_prompt_tail:");
            command.push_str(negative_prompt_tail);
        };

        command.push_str("\nsampler:");
        command.push_str(&self.sampler);
        command.push_str("\nscheduler:");
        command.push_str(&self.scheduler);

        if !self.model.is_empty() {
            command.push_str("\nmodel:");
            command.push_str(&self.model);
        }

        if let Some(vae) = &self.vae {
            command.push_str("\nvae:");
            command.push_str(vae);
        };

        command.push_str("\nsteps:");
        command.push_str(&self.steps.to_string());
        command.push_str(&format!("\nsize:{}x{}", self.width, self.height));
        command.push_str("\nclip_skip:");
        command.push_str(&self.clip_skip.to_string());
        command.push_str("\ncfg_scale:");
        command.push_str(&self.cfg_scale.to_string());

        command
    }
}
