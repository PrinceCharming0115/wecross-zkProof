#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, ProofMsg, QueryMsg};
use crate::state::{Epoch, Witness};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:reclaim-cosmwasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

// These functions run in the cosmwasm VM context and not in contract to be performant
extern "C" {
    fn addr_humanize(source_ptr: u32, destination_ptr: u32) -> u32;
    fn secp256k1_verify(message_hash_ptr: u32, signature_ptr: u32, public_key_ptr: u32) -> u32;
    fn secp256k1_recover_pubkey(
        message_hash_ptr: u32,
        signature_ptr: u32,
        recovery_param: u32,
    ) -> u64;

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::VerifyProof(msg) => verify_proof(deps, msg, &info.sender.clone()),
    }
}

pub fn fetch_witness_for_claim(
    epoch: Epoch,
    identifier: u32,
    timestamp: Timestamp,
) -> Vec<Witness> {
    // Create a hash from identifier+epoch+minimum+timestamp
    vec![]
}

pub fn verify_proof(
    deps: DepsMut,
    msg: ProofMsg,
    sender: &Addr,
) -> Result<Response, ContractError> {
    // Find the epoch from database

    // Construct a SignedClaims Object

    // Hash the claims, and verify with identifier hash

    // Fetch witness for claim

    // recover witness address from SignedClaims Object

    // make sure the minimum requirement for witness is satisfied

    // Ensure for every signature in the sign, a expected witness exists from the database
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
