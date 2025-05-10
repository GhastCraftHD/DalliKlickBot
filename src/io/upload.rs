use crate::game::Difficulty;
use crate::io;
use image::{ImageFormat, ImageReader};
use serde::{Deserialize, Serialize};
use serenity::all::Attachment;
use std::path::PathBuf;
use surrealdb::Uuid;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct UploadMetaData {
    pub id: String,
    pub subject: String,
    pub path: PathBuf,
    pub difficulty: Difficulty,
}

pub struct UploadMetaDataBuilder {
    id: Option<String>,
    subject: Option<String>,
    path: Option<PathBuf>,
    difficulty: Option<Difficulty>,
}

impl UploadMetaDataBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            subject: None,
            path: None,
            difficulty: None,
        }
    }

    pub fn subject(mut self, subject: String) -> Self {
        self.subject = Some(subject);
        self
    }

    pub async fn file(
        mut self,
        attachment: Attachment,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let uuid = Uuid::new_v4().to_string();
        let image_dir = io::get_image_dir().await;
        let file_path = image_dir.join(format!("{}.png", uuid));

        let response = reqwest::get(&attachment.url).await?;
        let content = response.bytes().await?;

        let image = ImageReader::new(std::io::Cursor::new(content))
            .with_guessed_format()?
            .decode()?;

        image.save_with_format(&file_path, ImageFormat::Png)?;
        self.path = Some(file_path);
        self.id = Some(uuid);
        Ok(self)
    }

    pub fn difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = Some(difficulty);
        self
    }

    pub fn build(self) -> UploadMetaData {
        UploadMetaData {
            id: self.id.expect("The ID of this Dalli Klick"),
            subject: self.subject.expect("The subject of this Dalli Klick"),
            path: self.path.expect("The file path of the locally stored image"),
            difficulty: self.difficulty.expect("The difficulty of the Dalli Klick"),
        }
    }
}
