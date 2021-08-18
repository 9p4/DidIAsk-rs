use serenity::prelude::*;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq)]
pub enum GlobalKeys {
    ChannelWhitelist,
    GulagRole,
    StatusPrefix,
    StatusPhrase,
}

pub struct GlobalInformation;

impl TypeMapKey for GlobalInformation {
    type Value = HashMap<GlobalKeys, Vec<String>>;
}
