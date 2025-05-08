use std::sync::Arc;

use dalli_klick_bot::config::Config;
use dalli_klick_bot::handlers::Handler;
use tracing::{info};
use dalli_klick_bot::context::Holder;
use dalli_klick_bot::database::init_database;
use dalli_klick_bot::{bot, logging};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {    
    logging::init_logging()?;
    info!("Starting DalliKlick Bot...");
    
    let config = Config::load_config()
        .expect("Could not load config");
    let database = init_database(&config).await
        .expect("Could not initialise database");
    
    let handler = Handler::new(Arc::new(Holder::new(config, database)));
    
    bot::init_bot(handler).await;
    
    Ok(())    
}
