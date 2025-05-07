use std::sync::Arc;
use serenity::all::{ActivityData, Context, EventHandler, Interaction, Ready};
use serenity::async_trait;
use tracing::info;
use crate::config::Config;
use crate::command;

pub struct Handler {
    pub config: Arc<Config>,
}

impl Handler {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected to Discord!", ready.user.name);

        ctx.set_activity(Some(ActivityData::playing(&self.config.bot.status)));
        
        command::register(&ctx, self.config.specification.guild_id).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            command::handle(&ctx, &command).await;
        }
    }
}