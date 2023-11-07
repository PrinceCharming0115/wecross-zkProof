use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};

#[cw_serde]
pub struct Witness {
    pub address: Addr,
    pub host: String,
}

#[cw_serde]
pub struct Epoch {
    pub id: Uint128,
    pub timestamp_start: Timestamp,
    pub timestamp_end: Timestamp,
    pub minimum_witness_for_claim_creation: Uint128,
    pub witness: Vec<Witness>,
}

pub const EPOCHS: cw_storage_plus::Map<Uint128, Epoch> = cw_storage_plus::Map::new("epocs");
