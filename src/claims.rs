use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};

#[cw_serde]
pub struct ClaimInfo {
    pub provider: String,
    pub parameters: String,
    pub context: String,
}

impl ClaimInfo {
    pub fn hash(&self) -> u32 {
        unimplemented!();
    }
}

#[cw_serde]
pub struct CompleteClaimData {
    pub identifier: Uint128,
    pub owner: Addr,
    pub epoch: Uint128,
    pub timestamp_s: Timestamp,
}

impl CompleteClaimData {
    pub fn serialise() {
        unimplemented!();
    }
}

#[cw_serde]
pub struct SignedClaim {
    pub claim: CompleteClaimData,
    pub bytes: Vec<u32>,
}

impl SignedClaim {
    pub fn recover_signers_of_signed_claim(&self) -> Vec<Addr> {
        unimplemented!();
    }
}
