use std::sync::Arc;
use serenity::prelude::TypeMapKey;
use crate::config::Config;

pub struct ConfigKey;
impl TypeMapKey for ConfigKey {
    type Value = Arc<Config>;
}