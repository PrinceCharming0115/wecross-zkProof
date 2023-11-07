#[cfg(not(feature = "library"))]
#[cfg(feature = "vanilla")]
use {
    crate::state_vanilla::{CONFIG, EPOCHS},
    cosmwasm_std::entry_point,
    cosmwasm_std::{
        Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp, Uint128,
    },
};

#[cfg(feature = "secret")]
use {
    crate::state_secret::{CONFIG, EPOCHS},
    secret_std::{
        entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
        StdResult, Timestamp, Uint128,
    },
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, ProofMsg, QueryMsg};
use crate::state::{Config, Epoch, Witness};
use sha2::{Digest, Sha256};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:reclaim-cosmwasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

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
        ExecuteMsg::VerifyProof(msg) => verify_proof(deps, msg, env),
        ExecuteMsg::AddEpoch {
            witness,
            minimum_witness,
        } => add_epoch(deps, env, witness, minimum_witness, info.sender.clone()),
    }
}

fn generate_random_seed(bytes: Vec<u8>, offset: usize) -> u32 {
    // Convert the hash result into a u32 using the offset
    let hash_slice = &bytes[offset..offset + 4];
    let mut seed = 0u32;
    for (i, &byte) in hash_slice.iter().enumerate() {
        seed |= u32::from(byte) << (i * 8);
    }

    seed
}

pub fn fetch_witness_for_claim(
    epoch: Epoch,
    identifier: Vec<u8>,
    timestamp: Timestamp,
) -> Vec<Witness> {
    let mut selected_witness = vec![];

    // Create a hash from identifier+epoch+minimum+timestamp
    let hash_str = format!(
        "{}\n{}\n{}\n{}",
        hex::encode(identifier),
        epoch.minimum_witness_for_claim_creation.to_string(),
        timestamp.nanos().to_string(),
        epoch.id.to_string()
    );
    let result = hash_str.as_bytes().to_vec();
    let mut hasher = Sha256::new();
    hasher.update(result);
    let hash_result = hasher.finalize().to_vec();
    let witenesses_left_list = epoch.witness;
    let mut byte_offset = 0;
    let witness_left = witenesses_left_list.len();
    for _i in 0..epoch.minimum_witness_for_claim_creation.into() {
        let random_seed = generate_random_seed(hash_result.clone(), byte_offset) as usize;
        let witness_index = random_seed % witness_left;
        let witness = witenesses_left_list.get(witness_index);
        match witness {
            Some(data) => selected_witness.push(data.clone()),
            None => {}
        }
        byte_offset = (byte_offset + 4) % hash_result.len();
    }

    selected_witness
}

#[cfg(feature = "vanilla")]
pub fn verify_proof(deps: DepsMut, msg: ProofMsg, env: Env) -> Result<Response, ContractError> {
    // Find the epoch from database
    let epoch = EPOCHS.load(deps.storage, msg.signed_claim.claim.epoch.into())?;

    // Hash the claims, and verify with identifier hash
    let hashed = msg.claim_info.hash();
    if msg.signed_claim.claim.identifier != hashed {
        return Err(ContractError::HashMismatchErr {});
    }

    // Fetch witness for claim
    let expected_witness = fetch_witness_for_claim(
        epoch,
        msg.signed_claim.claim.identifier.clone(),
        env.block.time,
    );

    let expected_witness_addresses = Witness::get_addresses(expected_witness);

    // recover witness address from SignedClaims Object
    let signed_witness = msg.signed_claim.recover_signers_of_signed_claim(deps)?;

    // make sure the minimum requirement for witness is satisfied
    if expected_witness_addresses.len() != signed_witness.len() {
        return Err(ContractError::WitnessMismatchErr {});
    }

    // Ensure for every signature in the sign, a expected witness exists from the database
    for signed in signed_witness {
        if !expected_witness_addresses.contains(&signed) {
            return Err(ContractError::SignatureErr {});
        }
    }
    Ok(Response::default())
}

#[cfg(feature = "vanilla")]
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
