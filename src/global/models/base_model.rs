use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use serde_json::Value;
use serenity::all::{CreateAttachment, CreateEmbed, EmbedMessageBuilding, MessageBuilder};
use tokio::sync::RwLock;

use crate::discord::commands::utilities::copy_modal;

use super::{civitai_info::CivitaiInfo, preview::ModelPreview};

#[derive(Debug, Clone)]
pub struct Model {
    pub civitai_info: Option<CivitaiInfo>,
    pub internal_name: String,
    pub preview: Option<ModelPreview>,
}

pub type Models = Arc<RwLock<Vec<Model>>>;

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

impl copy_modal::CopyButtonId for Model {
    fn id(&self) -> String {
        self.internal_name.clone()
    }
}

pub async fn get() -> Vec<Model> {
    let client = reqwest::ClientBuilder::new()
        .build()
        .expect("Could not create HTTP client");

    let mut url = dotenvy::var("A1111_BASE_URL").expect("Expected a URL to access A1111");
    url.push_str("/sdapi/v1/sd-models");
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
