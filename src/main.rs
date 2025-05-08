use std::sync::Arc;

use crate::config::Config;
use crate::handlers::Handler;
use tracing::{info};
use crate::context::Holder;
use crate::database::init_database;

mod logging;
mod config;
mod handlers;
mod command;
mod game;
mod database;
mod context;
mod bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting DalliKlick Bot...");
    
    logging::init_logging()?;
    
    let config = Config::load_config()
        .expect("Could not load config");
    let database = init_database(&config).await
        .expect("Could not initialise database");
    
    let handler = Handler::new(Arc::new(Holder {config, database}));
    
    bot::init_bot(handler).await;
    
    Ok(())    
}
