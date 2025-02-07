use std::collections::HashMap;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serenity::all::MessageBuilder;
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};

#[derive(Debug, Serialize, Deserialize)]
struct DrawRequest {
    prompt: String,
    negative_prompt: String,
    steps: u8,
}

impl Default for DrawRequest {
    fn default() -> Self {
        Self {
            prompt: "".to_string(),
            negative_prompt: "".to_string(),
            steps: 1,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct DrawResponse {
    images: Vec<String>,
    info: String,
    parameters: HashMap<String, Value>,
}

impl<'a> Into<DrawRequest> for &[ResolvedOption<'a>] {
    fn into(self) -> DrawRequest {
        let mut request = DrawRequest::default();

        for option in self.iter() {
            match option {
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "prompt",
                    ..
                } => request.prompt = value.to_string(),
                ResolvedOption {
                    value: ResolvedValue::String(value),
                    name: "negative_prompt",
                    ..
                } => request.negative_prompt = value.to_string(),
                ResolvedOption {
                    value: ResolvedValue::Integer(value),
                    name: "steps",
                    ..
                } => request.steps = *value as u8,
                _ => {}
            }
        }

        request
    }
}

pub async fn run<'a>(options: &[ResolvedOption<'a>]) -> (String, Option<Vec<u8>>) {
    println!("{options:?}");

    let gen_request: DrawRequest = options.into();

    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("Could not create HTTP client");

    let mut url = dotenv::var("A1111_BASE_URL").expect("Expected a URL to access A1111");
    url.push_str("/sdapi/v1/txt2img");

    let request = client.post(url).json(&gen_request).build().unwrap();

    let response: DrawResponse = client.execute(request).await.unwrap().json().await.unwrap();
    // println!("{}", response_string);

    // let respose: DrawResponse = serde_json::from_str(&response_string).unwrap();

    let content = MessageBuilder::new()
        .push_bold_line_safe("Prompt:")
        .push_codeblock_safe(gen_request.prompt, None)
        .push_bold_line("Negative prompt:")
        .push_codeblock_safe(gen_request.negative_prompt, None)
        .build();

    let image = response
        .images
        .into_iter()
        .next()
        .map(|data| BASE64_STANDARD.decode(data).expect("Error decoding image"));

    (content, image)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("draw")
        .description("draw an image")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "prompt", "The prompt to use")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "negative_prompt",
                "The negative_prompt to use",
            )
            .required(false),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "steps",
                "The steps to use, default 10",
            )
            .required(false),
        )
}
