use std::path::PathBuf;
use thiserror::Error;
use tokio::fs;
pub mod upload;

#[derive(Debug, Error)]
pub enum IoError {
    #[error(transparent)]
    Upload(#[from] upload::UploadIoError)
}

pub async fn get_app_dir() -> PathBuf {
    let app_dir = dirs::home_dir()
        .expect("Could not determine home directory")
        .join(".dalliklick");

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)
            .await
            .expect("Could not create application directory");
    }

    app_dir
}

pub async fn get_image_dir() -> PathBuf {
    let image_dir = get_app_dir().await.join("images");

    if !image_dir.exists() {
        fs::create_dir_all(&image_dir)
            .await
            .expect("Could not create image directory");
    }

    image_dir
}
