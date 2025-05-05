use std::sync::Arc;

use serenity::{
    async_trait,
    model::prelude::*,
    prelude::*,
    Client,
};
use serenity::all::ActivityData;
use tracing::{error, info};
use crate::config::Config;
use crate::context::ConfigKey;

mod logging;
mod config;
mod context;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected to Discord!", ready.user.name);
        
        let data = ctx.data.read().await;
        let config = data.get::<ConfigKey>().expect("Config is missing").clone();
        
        ctx.set_activity(Some(ActivityData::playing(&config.bot.status)));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting DalliKlick Bot...");
    
    logging::init_logging()?;
    
    let config = Arc::new(Config::load_config().expect("Could not load config"));
    
    info!("Connecting DalliKlick Bot to Discord...");
    
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(&config.bot.token, intents)
        .event_handler(Handler)
        .await
        .expect("Error while creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ConfigKey>(Arc::clone(&config));
    }
    
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
    
    Ok(())    
}
