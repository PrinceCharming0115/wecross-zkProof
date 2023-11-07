use std::u128;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub current_epoch: Uint128,
}

#[cw_serde]
pub struct Witness {
    pub address: Addr,
    pub host: String,
}

#[cw_serde]
pub struct Epoch {
    pub id: Uint128,
    pub timestamp_start: u64,
    pub timestamp_end: u64,
    pub minimum_witness_for_claim_creation: Uint128,
    pub witness: Vec<Witness>,
}

pub const EPOCHS: cw_storage_plus::Map<u128, Epoch> = cw_storage_plus::Map::new("epochs");
pub const CONFIG: cw_storage_plus::Item<Config> = cw_storage_plus::Item::new("config");
