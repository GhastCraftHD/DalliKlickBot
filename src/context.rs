use std::sync::Arc;
use serenity::prelude::TypeMapKey;
use crate::config::Config;
use crate::database::Database;

pub struct ConfigKey;
impl TypeMapKey for ConfigKey {
    type Value = Arc<Config>;
}

pub struct DatabaseKey;
impl TypeMapKey for DatabaseKey {
    type Value = Arc<Database>;
}