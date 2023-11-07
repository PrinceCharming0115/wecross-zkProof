use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "vanilla")]
use cosmwasm_std::Uint128;

#[cfg(feature = "secret")]
use secret_std::Uint128;

use crate::{
    claims::{ClaimInfo, SignedClaim},
    state::Witness,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    VerifyProof(ProofMsg),
    AddEpoch {
        witness: Vec<Witness>,
        minimum_witness: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProofMsg {
    pub claim_info: ClaimInfo,
    pub signed_claim: SignedClaim,
}
