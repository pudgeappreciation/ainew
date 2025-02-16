use serde::{Deserialize, Serialize};

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
