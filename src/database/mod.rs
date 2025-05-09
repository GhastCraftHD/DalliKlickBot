use crate::config::{DatabaseConfig};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing::info;

pub async fn connect(creds: &DatabaseConfig) -> Result<Surreal<Client>, surrealdb::Error> {
    info!("Connecting to database under host {}", &creds.host);
    let db = Surreal::new::<Ws>(&creds.host).await?;
    info!("Signing into database as user {}", &creds.user);
    db.signin(Root {
        username: &creds.user,
        password: &creds.pass,
    }).await?;
    Ok(db)
}

pub async fn query<T, F>(creds: &DatabaseConfig, query: F) -> Result<T, surrealdb::Error>
where
    F: FnOnce(&Surreal<Client>) -> T,
{
    let db = connect(creds).await?;
    let result = query(&db);
    info!("Disconnecting from database...");
    Ok(result)
}