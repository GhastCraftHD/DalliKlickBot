use crate::game::Difficulty;
use crate::io;
use image::{ImageFormat, ImageReader};
use serenity::all::Attachment;
use std::path::PathBuf;
use surrealdb::{Uuid};
use surrealdb::sql::Datetime;
use tracing::info;
use crate::database::upload::DatabaseMetaData;

pub struct DatabaseMetaDataBuilder {
    id: Option<String>,
    created_at: Datetime,
    subject: Option<String>,
    path: Option<PathBuf>,
    difficulty: Option<Difficulty>,
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
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let uuid = Uuid::new_v4().to_string();
        info!("Creating new Dalli Klick under UUID {}", &uuid);
        let image_dir = io::get_image_dir().await;
        let file_path = image_dir.join(format!("{}.png", uuid));

        info!("Downloading attached image from {}", &attachment.url);
        let response = reqwest::get(&attachment.url).await?;
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

    pub fn build(self) -> DatabaseMetaData {
        info!("Building upload meta data");
        DatabaseMetaData {
            id: self.id.expect("The ID of this Dalli Klick"),
            created_at: self.created_at,
            subject: self.subject.expect("The subject of this Dalli Klick"),
            path: self.path.expect("The file path of the locally stored image"),
            difficulty: self.difficulty.expect("The difficulty of the Dalli Klick"),
        }
    }
}
