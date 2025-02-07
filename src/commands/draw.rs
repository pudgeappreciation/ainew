use std::collections::HashMap;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use serenity::all::{
    CommandInteraction, CreateAttachment, CreateInteractionResponse,
    CreateInteractionResponseFollowup, CreateInteractionResponseMessage, EditInteractionResponse,
    MessageBuilder,
};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::prelude::*;

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

async fn send_initial_interaction_response(ctx: &Context, command: &CommandInteraction) {
    let initial_message = CreateInteractionResponseMessage::new().content("**Drawing image...**");
    let initial_response = CreateInteractionResponse::Message(initial_message);
    println!("Responding to slash command");
    if let Err(why) = command.create_response(&ctx.http, initial_response).await {
        println!("Cannot respond to slash command: {why}");
    }
}

async fn draw(draw_request: &DrawRequest) -> Result<DrawResponse, ()> {
    println!("Drawing image");
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("Could not create HTTP client");

    let mut url = dotenv::var("A1111_BASE_URL").expect("Expected a URL to access A1111");
    url.push_str("/sdapi/v1/txt2img");

    let request = client.post(url).json(&draw_request).build().unwrap();

    println!("Parsing response");
    let Ok(http_response) = client.execute(request).await else {
        return Err(());
    };

    let Ok(response) = http_response.json().await else {
        return Err(());
    };

    Ok(response)
}

async fn close_initial_interaction(ctx: &Context, command: &CommandInteraction) {
    println!("Closing command response");
    let content = MessageBuilder::new()
        .mention(&command.user.id)
        .push(", your drawing is complete.")
        .build();
    let edit = EditInteractionResponse::new().content(content);
    if let Err(why) = command.edit_response(&ctx.http, edit).await {
        println!("Cannot edit response: {why}");
    }
}

fn final_response_message(draw_response: DrawResponse) -> CreateInteractionResponseFollowup {
    let Some(image_string) = draw_response.images.into_iter().next() else {
        return CreateInteractionResponseFollowup::new().content("**No image returned :/**");
    };

    let Ok(image_data) = BASE64_STANDARD.decode(image_string) else {
        return CreateInteractionResponseFollowup::new().content("**Error decoding image :/**");
    };

    return CreateInteractionResponseFollowup::new()
        .content("**Your drawing! :D**")
        .add_file(CreateAttachment::bytes(image_data, "image.png"));
}

async fn send_final_response(
    ctx: &Context,
    command: &CommandInteraction,
    draw_response: DrawResponse,
) {
    println!("Sending image");
    let message = final_response_message(draw_response);
    if let Err(why) = command.create_followup(&ctx.http, message).await {
        println!("Cannot create response: {why}");
    }
}

async fn send_failure_response(ctx: &Context, command: &CommandInteraction) {
    let edit = EditInteractionResponse::new().content("**Drawing failed :(**");
    if let Err(why) = command.edit_response(&ctx.http, edit).await {
        println!("Cannot edit response: {why}");
    }
}

pub async fn run<'a>(ctx: Context, command: CommandInteraction) {
    send_initial_interaction_response(&ctx, &command).await;

    let draw_request: DrawRequest = command.data.options().as_slice().into();
    let draw_response = draw(&draw_request).await;

    match draw_response {
        Ok(response) => {
            close_initial_interaction(&ctx, &command).await;
            send_final_response(&ctx, &command, response).await
        }
        Err(_) => send_failure_response(&ctx, &command).await,
    }
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
