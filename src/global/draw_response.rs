use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DrawResponse {
    pub images: Vec<Vec<u8>>,
    pub info: String,
    pub parameters: HashMap<String, Value>,
}

async fn save_image(path: &str, data: &Vec<u8>) -> Result<(), tokio::io::Error> {
    let mut file = File::create(path).await?;
    file.write_all(data).await?;

    Ok(())
}

impl DrawResponse {
    pub async fn save_images(&self, id: u64) {
        if let Ok(output) = dotenvy::var("OUTPUT_PATH") {
            for (i, image) in self.images.iter().enumerate() {
                let path = format!("{}/{}_{}.png", output, id, i);
                let result = save_image(&path, image).await;

                println!("{:?}", result);
            }
        }
    }
}
