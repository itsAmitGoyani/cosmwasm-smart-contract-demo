use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Record {
    pub owner: Addr,
    pub resolver: Addr,
    pub ttl: u64,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
// pub struct State<'a> {
//     pub records: Map<'a, u32, Record>,
//     pub operators: Map<'a, Addr, Map<'a, Addr, bool>>,
// }

pub const RECORDS: Item<HashMap<String, Record>> = Item::new("records");
pub const OPERATORS: Item<HashMap<Addr, HashMap<Addr, bool>>> = Item::new("operators");
