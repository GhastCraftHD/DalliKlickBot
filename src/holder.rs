use crate::config::Config;
use serenity::prelude::{Context, TypeMapKey};
use std::sync::Arc;
use crate::game::Challenge;

#[derive(Debug)]
pub struct Holder {
    pub config: Arc<Config>,
    pub challenge: Option<Challenge>
}

impl Holder {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            challenge: None,
        }
    }
}

pub struct HolderKey;
impl TypeMapKey for HolderKey {
    type Value = Arc<Holder>;
}

pub async fn setup_context(ctx: &Context, holder: Arc<Holder>) {
    let mut data = ctx.data.write().await;
    data.insert::<HolderKey>(holder);
}