use std::sync::Arc;

use dalli_klick_bot::config::Config;
use dalli_klick_bot::handlers::Handler;
use tracing::{info};
use dalli_klick_bot::holder::Holder;
use dalli_klick_bot::{bot, logging};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging()?;
    info!("Starting DalliKlick Bot...");
    
    let config = Config::load_config()
        .expect("Could not load config");

    let handler = Handler::new(Arc::new(Holder::new(config)));

    bot::init_bot(handler).await;
    
    Ok(())    
}
