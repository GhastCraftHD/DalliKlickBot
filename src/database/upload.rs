use crate::config::DatabaseConfig;
use crate::database;
use crate::database::{DatabaseError, DatabaseRecord};
use crate::game::Difficulty;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use surrealdb::sql;


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct DatabaseMetaData {
    pub id: String,
    pub created_at: sql::Datetime,
    pub subject: String,
    pub path: PathBuf,
    pub difficulty: Difficulty,
}

pub async fn upload_data(
    config: &DatabaseConfig, 
    meta_data: &DatabaseMetaData
) -> Result<(), DatabaseError> {
    let _: Option<DatabaseRecord> = database::connect(config).await?
        .create(("dalliklick", &meta_data.id.to_string()))
        .content(meta_data.clone())
        .await?;
    
    Ok(())
}