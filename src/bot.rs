use serenity::all::GatewayIntents;
use serenity::Client;
use tracing::{error, info};
use crate::handlers::Handler;

pub async fn init_bot(handler: Handler) {
    info!("Connecting DalliKlick Bot to Discord...");

    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(&handler.holder.config.bot.token, intents)
        .event_handler(handler)
        .await
        .expect("Error while creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}