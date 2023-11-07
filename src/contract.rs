#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp, Uint128,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, ProofMsg, QueryMsg};
use crate::state::{Epoch, Witness, CONFIG, EPOCHS};

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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::VerifyProof(msg) => verify_proof(deps, msg, &info.sender.clone()),
        ExecuteMsg::AddEpoch {
            witness,
            minimum_witness,
        } => add_epoch(deps, env, witness, minimum_witness, info.sender.clone()),
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

// @dev - add epoch
pub fn add_epoch(
    deps: DepsMut,
    env: Env,
    witness: Vec<Witness>,
    minimum_witness: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    // load configs
    let mut config = CONFIG.load(deps.storage)?;

    // Check if sender is owner
    if config.owner != sender {
        return Err(ContractError::Unauthorized {});
    }

    //Increment Epoch number
    let new_epoch = config.current_epoch + Uint128::one();
    // Create the new epoch
    let epoch = Epoch {
        id: new_epoch,
        witness,
        timestamp_start: env.block.time.nanos(),
        timestamp_end: env.block.time.plus_days(1).nanos(),
        minimum_witness_for_claim_creation: minimum_witness,
    };

    // Upsert the new epoch into memory
    EPOCHS.update(
        deps.storage,
        new_epoch.into(),
        // we check if epoch with same id already exists for safety
        |existsting| match existsting {
            None => Ok(epoch),
            Some(..) => Err(ContractError::AlreadyExists {}),
        },
    )?;

    // Save the new epoch
    config.current_epoch = new_epoch;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
