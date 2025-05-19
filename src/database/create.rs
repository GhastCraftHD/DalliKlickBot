use crate::config::DatabaseConfig;
use crate::database;
use crate::database::upload::DatabaseMetaData;
use crate::database::DatabaseError;
use crate::database::DatabaseError::NoSuchRecordError;

pub async fn retrieve_latest(config: &DatabaseConfig) 
    -> Result<DatabaseMetaData, DatabaseError> {
    let mut response = database::connect(config).await?
        .query(r#"
            SELECT * FROM dalliklick ORDER BY created_at DESC LIMIT 1;
        "#).await?;
    
    let result: Option<DatabaseMetaData> = response.take(0)?;
    
    let record = result.ok_or(NoSuchRecordError)?;
    
    Ok(record)
}