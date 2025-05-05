use std::sync::Arc;

use crate::config::Config;
use crate::handlers::Handler;
use serenity::{
    model::prelude::*,
    Client,
};
use tracing::{error, info};

mod logging;
mod config;
mod handlers;
mod command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting DalliKlick Bot...");
    
    logging::init_logging()?;
    
    let config = Arc::new(Config::load_config().expect("Could not load config"));
    
    info!("Connecting DalliKlick Bot to Discord...");
    
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(&config.bot.token, intents)
        .event_handler(Handler::new(Arc::clone(&config)))
        .await
        .expect("Error while creating client");
    
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
    
    Ok(())    
}
