use std::sync::Arc;
use serenity::prelude::TypeMapKey;

pub struct ConfigKey;

impl TypeMapKey for ConfigKey {
    type Value = Arc<crate::config::Config>;
}