use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

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
    pub identifier: u32,
    pub owner: Addr,
    pub epoch: i32,
    pub timestamp_s: i32,
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
