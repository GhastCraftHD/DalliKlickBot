use serde::{Deserialize, Serialize};
use crate::handlers::Handler;
use surrealdb::engine::any;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use tokio;
use tracing::info;

pub async fn init_database(handler: &Handler) -> Result<(), surrealdb::Error> {
    
    info!("Connecting to database under user: {}", &handler.config.database.user);
    let db = any::connect(&handler.config.database.host).await?;

    db.use_ns(&handler.config.database.namespace).use_db(&handler.config.database.database).await?;

    db.signin(Root {
        username: &handler.config.database.user,
        password: &handler.config.database.pass,
    }).await?;

    info!("Successfully connected to database");
    Ok(())
}