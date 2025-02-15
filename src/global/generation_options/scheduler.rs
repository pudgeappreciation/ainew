use std::sync::Arc;

use serde_json::Value;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Scheduler {
    pub name: String,
}

pub type Schedulers = Arc<RwLock<Vec<Scheduler>>>;

pub fn scheduler_name(sampler_data: &serde_json::Map<String, Value>) -> Option<String> {
    Some(sampler_data.get("name")?.as_str()?.to_owned())
}

pub async fn get() -> Vec<Scheduler> {
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("Could not create HTTP client");

    let mut url = dotenvy::var("A1111_BASE_URL").expect("Expected a URL to access A1111");
    url.push_str("/sdapi/v1/schedulers");
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
        .filter_map(|sampler| scheduler_name(sampler))
        .map(|name| Scheduler { name })
        .collect()
}
