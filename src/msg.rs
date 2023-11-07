use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::claims::{ClaimInfo, SignedClaim};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub struct ProofMsg {
    pub claim_info: ClaimInfo,
    pub signed_claim: SignedClaim,
}
