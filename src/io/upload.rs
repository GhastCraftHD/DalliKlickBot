use crate::game::Difficulty;
use crate::io;
use image::ImageFormat::Png;
use image::ImageReader;
use serenity::all::Attachment;
use std::path::PathBuf;
use surrealdb::Uuid;

pub struct UploadMetaData {
    id: String,
    subject: String,
    path: PathBuf,
    difficulty: Difficulty,
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

    pub async fn file(mut self, attachment: &Attachment) -> Result<Self, Box<dyn std::error::Error>> {
        let uuid = Uuid::new_v4().to_string();
        let image_dir = io::get_image_dir().await;
        let file_path = image_dir.join(format!("{}.png", uuid));

        let response = reqwest::get(&attachment.url).await?;
        let content = response.bytes().await?;

        let image = ImageReader::new(std::io::Cursor::new(content))
            .with_guessed_format()?
            .decode()?;

        image.save_with_format(&file_path, Png)?;
        self.path = Some(file_path);
        self.id = Some(uuid);
        Ok(self)
    }

    pub fn difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = Some(difficulty);
        self
    }



}

