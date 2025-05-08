use crate::config::Config;
use crate::database::Database;
use serenity::prelude::{Context, TypeMapKey};
use std::sync::Arc;

#[derive(Debug)]
pub struct Holder {
    pub config: Config,
    pub database: Database,
}
pub struct HolderKey;
impl TypeMapKey for HolderKey {
    type Value = Arc<Holder>;
}


pub async fn setup_context(ctx: &Context, holder: Arc<Holder>) {
    let mut data = ctx.data.write().await;
    data.insert::<HolderKey>(holder);
}