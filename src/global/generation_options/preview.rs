use std::path::PathBuf;

use tokio::fs;

#[derive(Debug, Clone)]
pub struct ModelPreview {
    pub image: Vec<u8>,
    pub extension: String,
}

impl ModelPreview {
    pub async fn from_path(mut path: PathBuf) -> Option<Self> {
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
