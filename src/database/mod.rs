use serde::Deserialize;
use crate::config::DatabaseConfig;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{RecordId, Surreal};
use thiserror::Error;
use tracing::info;

pub mod upload;
mod create;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    SurrealError(#[from] surrealdb::Error)
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseRecord {
    pub id: RecordId,
}

pub async fn connect(creds: &DatabaseConfig) -> Result<Surreal<Client>, DatabaseError> {
    info!("Connecting to database under host {}", &creds.host);
    let db = Surreal::new::<Ws>(&creds.host).await?;
    info!("Signing into database as user {}", &creds.user);
    db.signin(Root {
        username: &creds.user,
        password: &creds.pass,
    }).await?;
    db.use_ns(&creds.namespace).await?;
    db.use_db(&creds.database).await?;
    Ok(db)
}