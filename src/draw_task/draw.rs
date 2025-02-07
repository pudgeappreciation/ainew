use std::collections::HashMap;

use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs;

use crate::global::{draw_request::DrawRequest, draw_response::DrawResponse};

#[derive(Debug, Default, Serialize, Deserialize)]
struct DrawResponseBase64 {
    images: Vec<String>,
    info: String,
    parameters: HashMap<String, Value>,
}

impl Into<DrawResponse> for DrawResponseBase64 {
    fn into(self) -> DrawResponse {
        let mut images = Vec::new();
        for image in self.images {
            if let Ok(image) = BASE64_STANDARD.decode(image) {
                images.push(image);
            }
        }

        DrawResponse {
            images,
            info: self.info,
            parameters: self.parameters,
        }
    }
}

async fn draw_fake(fake_image_path: &str) -> Result<DrawResponse, ()> {
    let image = fs::read_to_string(fake_image_path).await.unwrap();
    let image = BASE64_STANDARD.decode(image).unwrap();
    return Ok(DrawResponse {
        images: vec![image],
        info: "".to_string(),
        parameters: HashMap::new(),
    });
}

pub async fn draw(draw_request: &DrawRequest) -> Result<DrawResponse, ()> {
    if let Ok(fake_image_path) = dotenvy::var("USE_FAKE_IMAGE") {
        return draw_fake(&fake_image_path).await;
    }

    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("Could not create HTTP client");

    let mut url = dotenvy::var("A1111_BASE_URL").expect("Expected a URL to access A1111");
    url.push_str("/sdapi/v1/txt2img");

    let request = client.post(url).json(&draw_request).build().unwrap();

    let Ok(http_response) = client.execute(request).await else {
        return Err(());
    };

    let Ok(response) = http_response.json::<DrawResponseBase64>().await else {
        return Err(());
    };

    let response: DrawResponse = response.into();
    response.save_images(draw_request.request_id.get()).await;

    Ok(response)
}
