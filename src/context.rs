use crate::config::Config;
use crate::database::Database;
use serenity::prelude::{Context, TypeMapKey};
use std::sync::Arc;

#[derive(Debug)]
pub struct Holder {
    pub config: Arc<Config>,
    pub database: Arc<Database>,
}

impl Holder {
    pub fn new(config: Config, database: Database) -> Self {
        Self {
            config: Arc::new(config),
            database: Arc::new(database),
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