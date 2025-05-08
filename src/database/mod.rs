use crate::config::Config;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::info;

#[derive(Debug)]
pub struct Database {
    pub db: Surreal<Client>,
}

pub async fn init_database(config: &Config) -> Result<Database, surrealdb::Error> {
    
    info!("Initialising database under host {}", &config.database.host);
    let database = Database {
        db: Surreal::new::<Ws>(&config.database.host).await?,
    };
    
    info!("Signing into database under user {}", &config.database.user);
    database.db.signin(Root {
        username: &config.database.user,
        password: &config.database.pass,
    }).await?;

    info!(
        "Using database {} in namespace {}",
        &config.database.database,
        &config.database.namespace
    );
    database.db.use_ns(&config.database.namespace).use_db(&config.database.database).await?;
    
    info!("Successfully initialised to database");
    Ok(database)
}