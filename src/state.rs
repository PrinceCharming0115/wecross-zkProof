use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "vanilla")]
use cosmwasm_std::{Addr, Uint128};

#[cfg(feature = "secret")]
use secret_std::{Addr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub owner: Addr,
    pub current_epoch: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Witness {
    pub address: String,
    pub host: String,
}

impl Witness {
    pub fn get_addresses(witness: Vec<Witness>) -> Vec<String> {
        let mut vec_addresses = vec![];
        for wit in witness {
            vec_addresses.push(wit.address);
        }
        return vec_addresses;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Epoch {
    pub id: Uint128,
    pub timestamp_start: u64,
    pub timestamp_end: u64,
    pub minimum_witness_for_claim_creation: Uint128,
    pub witness: Vec<Witness>,
}
