use crate::game::Difficulty;
use crate::io;
use image::{ImageFormat, ImageReader};
use serenity::all::Attachment;
use std::path::PathBuf;
use reqwest::header;
use reqwest::header::ToStrError;
use surrealdb::{Uuid};
use surrealdb::sql::Datetime;
use thiserror::Error;
use tracing::info;
use crate::database::upload::DatabaseMetaData;
use crate::io::upload::UploadIoError::BuildError;

pub struct DatabaseMetaDataBuilder {
    id: Option<String>,
    created_at: Datetime,
    subject: Option<String>,
    path: Option<PathBuf>,
    difficulty: Option<Difficulty>,
}

#[derive(Debug, Error)]
pub enum UploadIoError {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Invalid Content-Type: {0}")]
    InvalidContentType(String),

    #[error("Missing Content-Type header")]
    MissingContentType,

    #[error("Image decoding failed: {0}")]
    ImageDecoding(#[from] image::ImageError),

    #[error("Failed to parse header value: {0}")]
    HeaderToStr(#[from] ToStrError),

    #[error("Failed to save image: {0}")]
    ImageSave(#[from] std::io::Error),

    #[error("Failed to convert file path to string")]
    PathToStr,

    #[error("Missing attribute {0} while building")]
    BuildError(String),

}

impl DatabaseMetaDataBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            created_at: Datetime::default(),
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
    ) -> Result<Self, UploadIoError> {
        let uuid = Uuid::new_v4().to_string();
        info!("Creating new Dalli Klick under UUID {}", &uuid);
        let image_dir = io::get_image_dir().await;
        let file_path = image_dir.join(format!("{}.png", uuid));

        info!("Downloading attached file from {}", &attachment.url);
        let response = reqwest::get(&attachment.url).await?;

        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .ok_or(UploadIoError::MissingContentType)?;

        let mime = content_type.to_str()?;

        if !mime.starts_with("image/") {
            return Err(UploadIoError::InvalidContentType(mime.to_string()));
        }

        let content = response.bytes().await?;

        let image = ImageReader::new(std::io::Cursor::new(content))
            .with_guessed_format()?
            .decode()?;

        image.save_with_format(&file_path, ImageFormat::Png)?;
        info!(
            "Saved attached image under {}",
            &file_path.to_str().expect("Error while decoding file path")
        );
        self.path = Some(file_path);
        self.id = Some(uuid);
        Ok(self)
    }

    pub fn difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = Some(difficulty);
        self
    }

    pub fn build(self) -> Result<DatabaseMetaData, UploadIoError> {
        info!("Building upload meta data");
        Ok(DatabaseMetaData {
            id: self.id.ok_or(BuildError("id".to_string()))?,
            created_at: self.created_at,
            subject: self.subject.ok_or(BuildError("subject".to_string()))?,
            path: self.path.ok_or(BuildError("path".to_string()))?,
            difficulty: self.difficulty.ok_or(BuildError("difficulty".to_string()))?,
        })
    }
}
