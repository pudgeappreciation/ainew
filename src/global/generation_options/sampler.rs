use std::sync::Arc;

use serde_json::Value;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Sampler {
    pub name: String,
}

pub type Samplers = Arc<RwLock<Vec<Sampler>>>;

pub fn sampler_name(sampler_data: &serde_json::Map<String, Value>) -> Option<String> {
    Some(sampler_data.get("name")?.as_str()?.to_owned())
}

pub async fn get() -> Vec<Sampler> {
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("Could not create HTTP client");

    let mut url = dotenvy::var("A1111_BASE_URL").expect("Expected a URL to access A1111");
    url.push_str("/sdapi/v1/samplers");
    let request = client.get(url).build().unwrap();

    let Ok(http_response) = client.execute(request).await else {
        return Vec::new();
    };

    let Ok(response) = http_response
        .json::<Vec<serde_json::Map<String, Value>>>()
        .await
    else {
        return Vec::new();
    };

    response
        .iter()
        .filter_map(|sampler| sampler_name(sampler))
        .map(|name| Sampler { name })
        .collect()
}
