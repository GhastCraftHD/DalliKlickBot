use std::sync::Arc;

use crate::config::Config;
use crate::handlers::Handler;
use serenity::{
    model::prelude::*,
    Client,
};
use tracing::{error, info};
use crate::context::Holder;
use crate::database::init_database;

mod logging;
mod config;
mod handlers;
mod command;
mod game;
mod database;
mod context;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting DalliKlick Bot...");
    
    logging::init_logging()?;
    
    let config = Config::load_config()
        .expect("Could not load config");
    let database = init_database(&config).await
        .expect("Could not initialise database");
    
    let handler = Handler::new(Arc::new(Holder {config, database}));
    
    info!("Connecting DalliKlick Bot to Discord...");
    
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(&handler.holder.config.bot.token, intents)
        .event_handler(handler)
        .await
        .expect("Error while creating client");
    
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
    
    Ok(())    
}
