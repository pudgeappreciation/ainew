use serde::{Deserialize, Serialize};

use super::options::Options;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOverride {
    pub sd_model_checkpoint: String,
    pub sd_vae: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOptions {
    pub prompt: String,
    pub negative_prompt: String,
    pub sampler: String,
    pub scheduler: String,
    pub override_settings: ApiOverride,
    pub steps: u8,
    pub seed: u32,
    pub width: u32,
    pub height: u32,
    pub clip_skip: u8,
    pub cfg_scale: f32,
}

impl From<Options> for ApiOptions {
    fn from(value: Options) -> Self {
        Self {
            prompt: value.prompt,
            negative_prompt: value.negative_prompt,
            sampler: value.sampler,
            scheduler: value.scheduler,
            steps: value.steps,
            seed: value.seed,
            width: value.width,
            height: value.height,
            clip_skip: value.clip_skip,
            cfg_scale: value.cfg_scale,
            override_settings: ApiOverride {
                sd_model_checkpoint: value.model,
                sd_vae: value.vae,
            },
        }
    }
}
