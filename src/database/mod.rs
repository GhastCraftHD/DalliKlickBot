use surrealdb::engine::remote::ws::Ws;
use crate::handlers::Handler;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::info;

pub struct Database;

pub async fn init_database(handler: &Handler) -> Result<(), surrealdb::Error> {
    
    info!("Connecting to database under user: {}", &handler.config.database.user);
    let db = Surreal::new::<Ws>(&handler.config.database.host).await?;

    db.use_ns(&handler.config.database.namespace).use_db(&handler.config.database.database).await?;

    db.signin(Root {
        username: &handler.config.database.user,
        password: &handler.config.database.pass,
    }).await?;

    info!("Successfully connected to database");
    Ok(())
}