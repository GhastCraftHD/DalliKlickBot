use crate::command;
use crate::context::{setup_context, Holder};
use serenity::all::{ActivityData, Context, EventHandler, Interaction, Ready};
use serenity::async_trait;
use std::sync::Arc;
use tracing::info;

pub struct Handler {
    pub holder: Arc<Holder>,
}

impl Handler {
    pub fn new(holder: Arc<Holder>) -> Self {
        Self { holder }
    }
}

#[async_trait]
impl EventHandler for Handler {
    
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected to Discord and ready!", ready.user.name);

        ctx.set_activity(Some(ActivityData::playing(&self.holder.config.bot.status)));
        
        command::register(&ctx, self.holder.config.specification.guild_id).await;
        
        setup_context(&ctx, self.holder.clone()).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            command::handle(&ctx, &command).await;
        }
    }
}