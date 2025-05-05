use std::sync::Arc;
use tracing::info;
use crate::config::Config;

mod logging;
mod config;
mod context;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging()?;
    
    let config = Arc::new(Config::load_config().expect("Could not load config"));
    
    info!("Starting DalliKlick Bot...");
    
    Ok(())    
}
