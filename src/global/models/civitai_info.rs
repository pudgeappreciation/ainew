use std::path::PathBuf;

use tokio::fs;

#[derive(Debug, Clone)]
pub struct CivitaiInfo {
    pub name: String,
    pub version: String,
    pub base_model: String,
    pub version_id: i64,
    pub model_id: i64,
}

impl CivitaiInfo {
    pub async fn from_path(mut path: PathBuf) -> Option<Self> {
        path.set_extension("");
        path.set_extension("civitai.info");

        let raw = fs::read_to_string(path).await.ok()?;
        let info: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&raw).ok()?;

        let version = info.get("name")?.as_str()?.to_string();
        let base_model = info.get("baseModel")?.as_str()?.to_string();
        let name = info.get("model")?.as_object()?.get("name")?.to_string();
        let version_id = info.get("id")?.as_i64()?;
        let model_id = info.get("modelId")?.as_i64()?;

        Some(Self {
            version,
            base_model,
            name,
            version_id,
            model_id,
        })
    }
}
