use serde::Deserialize;
use crate::config::DatabaseConfig;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{RecordId, Surreal};
use tracing::info;

pub mod upload;
mod create;

#[derive(Debug, Deserialize)]
pub struct DatabaseRecord {
    id: RecordId,
}

pub async fn connect(creds: &DatabaseConfig) -> Surreal<Client> {
    info!("Connecting to database under host {}", &creds.host);
    let db = Surreal::new::<Ws>(&creds.host).await.expect("Database under host");
    info!("Signing into database as user {}", &creds.user);
    db.signin(Root {
        username: &creds.user,
        password: &creds.pass,
    }).await.expect("Authentication in database");
    db.use_ns(&creds.namespace)
        .use_db(&creds.database).await.expect("Namespace and database not found");
    db
}