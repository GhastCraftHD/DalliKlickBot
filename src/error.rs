use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Io(#[from] crate::io::IoError),

    #[error(transparent)]
    Command(#[from] crate::command::CommandError),
    
    #[error(transparent)]
    Database(#[from] crate::database::DatabaseError),
    
    #[error(transparent)]
    Game(#[from] crate::game::GameError),

    #[error("Failed to access shared application data")]
    SharedDataAccessError,

    #[error("Other error: {0}")]
    Other(String),

}