use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const STREAKS: Map<Addr, u32> = Map::new("streaks");
pub const LAST_CLAIMED: Map<Addr, u32> = Map::new("last_claimed");
