use crate::config::Config;
use serenity::prelude::{Context, TypeMapKey};
use std::sync::Arc;
use crate::error::AppError;
use crate::error::AppError::SharedDataAccessError;
use crate::game::Challenge;

#[derive(Debug)]
pub struct Holder {
    pub config: Arc<Config>,
    pub challenge: Arc<Option<Challenge>>
}

impl Holder {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            challenge: Arc::new(None),
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

pub async fn retrieve_holder(ctx: &Context) -> Result<Arc<Holder>, AppError> {
    let data = ctx.data.read().await;
    data.get::<HolderKey>()
        .cloned()
        .ok_or(SharedDataAccessError)
}