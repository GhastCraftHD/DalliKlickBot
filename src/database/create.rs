use crate::config::DatabaseConfig;
use crate::database;
use crate::database::DatabaseError;
use crate::database::upload::DatabaseMetaData;

pub async fn retrieve_latest(config: &DatabaseConfig) 
    -> Result<DatabaseMetaData, DatabaseError> {
    let response: Vec<DatabaseMetaData> = database::connect(config).await?
        .query(r#"
            SELECT * FROM dalliklick ORDER BY created_at DESC LIMIT 1;
        "#).await?
        .take(0)?;
    
    Ok(response.first().expect("The latest database record").clone())
}