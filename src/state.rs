use std::iter::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use serde_json::map;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Record {
    pub(crate) owner: Addr,
    pub(crate) resolver: Addr,
    pub(crate) ttl: u64,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub records: Map<u32, Record>,
    pub operators: map::Map<Addr, map::Map<Addr, bool>>,
}

pub const STATE: Item<State> = Item::new("state");