use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use serenity::{
    all::{CreateAttachment, CreateEmbed, EmbedMessageBuilding, MessageBuilder},
    json::Value,
};
use tokio::{fs, sync::RwLock};

#[derive(Debug, Clone)]
pub struct ModelPreview {
    pub image: Vec<u8>,
    pub extension: String,
}

#[derive(Debug, Clone)]
pub struct CivitaiInfo {
    pub name: String,
    pub version: String,
    pub base_model: String,
    pub version_id: i64,
    pub model_id: i64,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub civitai_info: Option<CivitaiInfo>,
    pub internal_name: String,
    pub preview: Option<ModelPreview>,
}

pub type Models = Arc<RwLock<Vec<Model>>>;

impl CivitaiInfo {
    async fn from_path(mut path: PathBuf) -> Option<Self> {
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

impl ModelPreview {
    async fn from_path(mut path: PathBuf) -> Option<Self> {
        path.set_extension("");

        path.set_extension("preview.png");
        let image = fs::read(path.clone()).await.ok();

        if let Some(preview) = image {
            return Some(Self {
                image: preview,
                extension: String::from("png"),
            });
        }

        path.set_extension("jpeg");
        Some(Self {
            image: fs::read(path).await.ok()?,
            extension: String::from("jpeg"),
        })
    }
}

impl Model {
    pub fn attachment(&self) -> Option<CreateAttachment> {
        let Some(preview) = self.preview.clone() else {
            return None;
        };

        Some(CreateAttachment::bytes(
            preview.image,
            format!("{}.{}", self.internal_name, preview.extension),
        ))
    }

    pub fn embed(&self) -> CreateEmbed {
        let mut embed = CreateEmbed::new();

        embed = match self.civitai_info.clone() {
            Some(civitai_info) => embed.title(civitai_info.name),
            None => embed.title(self.internal_name.clone()),
        };

        let mut content = MessageBuilder::new();
        content
            .push_bold_safe("Name: ")
            .push_safe(self.internal_name.clone());
        if let Some(civitai_info) = self.civitai_info.clone() {
            content
                .push_line("")
                .push_line("")
                .push_bold_safe("Source: ")
                .push_named_link_safe(
                    "civitai.com",
                    format!(
                        "https://civitai.com/models/{}?modelVersionId={}",
                        civitai_info.model_id, civitai_info.version_id,
                    ),
                )
                .push_line("")
                .push_bold_safe("Version: ")
                .push_safe(format!("{}", civitai_info.version))
                .push_line("")
                .push_bold_safe("Base model: ")
                .push_safe(civitai_info.base_model.clone())
                .build();
        };
        embed = embed.description(content.build());

        if let Some(preview) = self.preview.clone() {
            embed = embed.image(format!(
                "attachment://{}.{}",
                self.internal_name, preview.extension
            ));
        };

        embed
    }

    async fn from_path(path: PathBuf) -> Option<Self> {
        let full_file_name = path.file_name()?;
        let internal_name = full_file_name.to_str()?.split('.').next()?;

        let model_info = CivitaiInfo::from_path(path.clone()).await;
        let preview = ModelPreview::from_path(path.clone()).await;

        Some(Self {
            internal_name: internal_name.to_string(),
            preview,
            civitai_info: model_info,
        })
    }
}

pub async fn get_models() -> Vec<Model> {
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("Could not create HTTP client");

    let mut url = dotenvy::var("A1111_BASE_URL").expect("Expected a URL to access A1111");
    url.push_str("/sdapi/v1/sd-models");
    let request = client.get(url).build().unwrap();

    let Ok(http_response) = client.execute(request).await else {
        return Vec::new();
    };

    // print!("\n\nmodels: {:?}\n\n", http_response.text().await);
    // panic!();

    let Ok(response) = http_response
        .json::<Vec<serde_json::Map<String, Value>>>()
        .await
    else {
        return Vec::new();
    };

    let mut models = Vec::new();
    for model_data in response.iter() {
        let Some(filename) = model_data.get("filename") else {
            continue;
        };

        let Some(filename) = filename.as_str() else {
            continue;
        };

        let Some(model) = Model::from_path(Path::new(filename).to_path_buf()).await else {
            continue;
        };

        models.push(model);
    }

    models
}
