use serenity::{client::bridge::gateway::ShardManager, prelude::*};
use std::sync::Arc;

extern crate chrono;
use chrono::{DateTime, Utc};
use sysinfo::System;

pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct StartTimeContainer;
impl TypeMapKey for StartTimeContainer {
    type Value = DateTime<Utc>;
}

pub struct SysInfoContainer;
impl TypeMapKey for SysInfoContainer {
    type Value = System;
}
