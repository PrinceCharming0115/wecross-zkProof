#[cfg(not(feature = "library"))]
#[cfg(feature = "vanilla")]
use {
    crate::state_vanilla::{get_all_epochs, CONFIG, EPOCHS},
    cosmwasm_std::entry_point,
    cosmwasm_std::to_json_binary,
    cosmwasm_std::{
        Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp, Uint128,
    },
};

// use cw2::set_contract_version;
#[cfg(feature = "secret")]
use {
    crate::state_secret::{CONFIG, EPOCHS},
    secret_std::{
        entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
        StdResult, Timestamp, Uint128,
    },
};

use crate::state::{Epoch, Witness};
use crate::{error::ContractError, msg::GetAllEpochResponse};
use crate::{
    msg::{ExecuteMsg, GetEpochResponse, GetOwnerResponse, InstantiateMsg, ProofMsg, QueryMsg},
    state::Config,
};
use sha2::{Digest, Sha256};

// version info for migration info
// const CONTRACT_NAME: &str = "crates.io:reclaim-cosmwasm";
// const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let addr = deps.api.addr_validate(&msg.owner)?;
    let config = Config {
        owner: addr,
        current_epoch: Uint128::zero(),
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
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
    for _i in 0..epoch.minimum_witness_for_claim_creation {
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

    // Fetch witness for claim
    let _expected_witness = fetch_witness_for_claim(
        epoch,
        msg.signed_claim.claim.identifier.clone(),
        env.block.time,
    );

    let _expected_witness_addresses = Witness::get_addresses(_expected_witness);

    // recover witness address from SignedClaims Object
    let signed = msg.signed_claim.recover_signers_of_signed_claim(deps);

    if !signed {
        return Err(ContractError::SignatureErr {});
    }
    Ok(Response::default())
}

#[cfg(feature = "secret")]
pub fn verify_proof(deps: DepsMut, msg: ProofMsg, env: Env) -> Result<Response, ContractError> {
    // Find the epoch from database
    let fetched_epoch = EPOCHS.get(deps.storage, &msg.signed_claim.claim.epoch.into());

    match fetched_epoch {
        Some(epoch) => {
            // Fetch witness for claim
            let _expected_witness = fetch_witness_for_claim(
                epoch,
                msg.signed_claim.claim.identifier.clone(),
                env.block.time,
            );

            let _expected_witness_addresses = Witness::get_addresses(_expected_witness);

            // recover witness address from SignedClaims Object
            let signed = msg.signed_claim.recover_signers_of_signed_claim(deps);

            if !signed {
                return Err(ContractError::SignatureErr {});
            }
        }
        None => return Err(ContractError::NotFoundErr {}),
    }

    Ok(Response::default())
}

#[cfg(feature = "vanilla")]
// @dev - add epoch
pub fn add_epoch(
    deps: DepsMut,
    env: Env,
    witness: Vec<Witness>,
    minimum_witness: u64,
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

#[cfg(feature = "secret")]
// @dev - add epoch
pub fn add_epoch(
    deps: DepsMut,
    env: Env,
    witness: Vec<Witness>,
    minimum_witness: u64,
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
        timestamp_end: env.block.time.plus_seconds(86400).nanos(),
        minimum_witness_for_claim_creation: minimum_witness,
    };

    // Upsert the new epoch into memory
    EPOCHS.insert(deps.storage, &new_epoch.into(), &epoch)?;

    // Save the new epoch
    config.current_epoch = new_epoch;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

#[cfg(feature = "vanilla")]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetEpoch { id } => to_json_binary(&query_epoch_id(deps, id)?),
        QueryMsg::GetAllEpoch {} => to_json_binary(&query_all_epoch_ids(deps)?),
        QueryMsg::GetOwner {} => to_json_binary(&query_owner(deps)?),
    }
}

#[cfg(feature = "secret")]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetEpoch { id } => to_binary(&query_epoch_id(deps, id)?),
        QueryMsg::GetAllEpoch {} => to_binary(&query_all_epoch_ids(deps)?),
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
    }
}

#[cfg(feature = "vanilla")]
fn query_all_epoch_ids(deps: Deps) -> StdResult<GetAllEpochResponse> {
    Ok(GetAllEpochResponse {
        ids: get_all_epochs(deps.storage)?,
    })
}

#[cfg(feature = "vanilla")]
fn query_epoch_id(deps: Deps, id: u128) -> StdResult<GetEpochResponse> {
    let data = EPOCHS.load(deps.storage, id)?;
    Ok(GetEpochResponse { epoch: data })
}

#[cfg(feature = "vanilla")]
fn query_owner(deps: Deps) -> StdResult<GetOwnerResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(GetOwnerResponse {
        owner: config.owner.to_string(),
    })
}

//NOTE: Unimplemented as secret doesn't allow to iterate via keys
#[cfg(feature = "secret")]
fn query_all_epoch_ids(_deps: Deps) -> StdResult<GetAllEpochResponse> {
    Ok(GetAllEpochResponse { ids: vec![] })
}

#[cfg(feature = "secret")]
fn query_epoch_id(deps: Deps, id: u128) -> StdResult<GetEpochResponse> {
    let data = EPOCHS.get(deps.storage, &id);
    match data {
        Some(epoch) => Ok(GetEpochResponse { epoch }),
        None => Err(StdError::NotFound {
            kind: "No such epoch".to_string(),
        }),
    }
}

#[cfg(feature = "secret")]
fn query_owner(deps: Deps) -> StdResult<GetOwnerResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(GetOwnerResponse {
        owner: config.owner.to_string(),
    })
}

#[cfg(test)]
mod tests {

    use crate::claims::{ClaimInfo, CompleteClaimData, SignedClaim};

    use super::*;
    use cosmwasm_std::{from_json, testing::*};
    use cosmwasm_std::{Coin, Uint128};
    use hex_literal::hex;

    pub fn keccak256(bytes: &[u8]) -> [u8; 32] {
        use tiny_keccak::{Hasher, Keccak};
        let mut output = [0u8; 32];
        let mut hasher = Keccak::v256();
        hasher.update(bytes);
        hasher.finalize(&mut output);
        output
    }

    #[test]

    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let mock_api = MockApi::default().with_prefix("secret1");

        let info = mock_info(
            mock_api.addr_make("owner").to_string().as_str(),
            &[Coin {
                denom: "earth".to_string(),
                amount: Uint128::new(1000),
            }],
        );

        let owner = info.clone().sender.into_string();
        let init_msg = InstantiateMsg {
            owner: owner.clone(),
        };

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: GetOwnerResponse = from_json(&res).unwrap();
        assert_eq!(owner, value.owner);
    }

    #[test]
    fn epoch_insertion() {
        let mut deps = mock_dependencies();
        let mock_api = MockApi::default().with_prefix("secret1");
        let owner_addr = mock_api.addr_make("owner");
        let info = mock_info(
            owner_addr.to_string().as_str(),
            &[Coin {
                denom: "earth".to_string(),
                amount: Uint128::new(1000),
            }],
        );

        let owner = info.clone().sender.into_string();
        let init_msg = InstantiateMsg {
            owner: owner.clone(),
        };

        let str_verifying_key = "0x76f6b994e78079940634f8c1c856f8a5b883259a".to_owned();

        let witness: Witness = Witness {
            address: str_verifying_key,
            host: "https://".to_string(),
        };
        let mut witness_vec = Vec::new();
        witness_vec.push(witness);
        instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();
        let execute_msg = ExecuteMsg::AddEpoch {
            witness: witness_vec,
            minimum_witness: 1_u64,
        };
        let res = execute(deps.as_mut(), mock_env(), info, execute_msg).unwrap();
        assert_eq!(0, res.messages.len());
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetEpoch { id: 1 }).unwrap();
        let value: GetEpochResponse = from_json(&res).unwrap();
        assert_eq!(Uint128::from(1_u128), value.epoch.id)
    }

    #[test]
    fn proof_verification() {
        let mut deps = mock_dependencies();
        let mock_api = MockApi::default().with_prefix("secret1");
        let owner_addr = mock_api.addr_make("owner");
        let info = mock_info(
            owner_addr.to_string().as_str(),
            &[Coin {
                denom: "earth".to_string(),
                amount: Uint128::new(1000),
            }],
        );

        let owner = info.clone().sender.into_string();
        let init_msg = InstantiateMsg {
            owner: owner.clone(),
        };

        let enc_key = "0x76f6b994e78079940634f8c1c856f8a5b883259a".to_owned();

        let witness: Witness = Witness {
            address: enc_key.clone(),
            host: "https://".to_string(),
        };
        let mut witness_vec = Vec::new();
        witness_vec.push(witness);

        instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        let add_epoch_msg = ExecuteMsg::AddEpoch {
            witness: witness_vec,
            minimum_witness: 1_u64,
        };
        execute(deps.as_mut(), mock_env(), info.clone(), add_epoch_msg).unwrap();

        let claim_info = ClaimInfo {
            provider: "provider".to_owned(),
            parameters: "params".to_owned(),
            context: "{}".to_owned(),
        };

        let hashed = claim_info.hash();
        let now = mock_env().block.time.seconds();
        let complete_claim_data = CompleteClaimData {
            identifier: hashed,
            owner: enc_key,
            epoch: 1_u64,
            timestamp_s: now,
        };

        let mut sigs = Vec::new();
        // let (signature, recid) = signing_key.sign_prehash_recoverable(&result).unwrap();
        // let enc = base16::encode_lower(&signature.to_bytes());
        // dbg!(&enc);
        // let dec = base16::decode(enc.as_bytes()).unwrap();
        // let recid_8: u8 = recid.try_into().unwrap();
        let msg = "0xa6db2030140d1a1297ea836cf1fb0a1b467c5c21499dc0cd08dba63d62a6fdcc\n0x58843824aea8ed7fa22d524d723c83e1f3b7bce0\n1708529323\n1".to_string();
        let mut eth_message = format!("\x19Ethereum Signed Message:\n{}", msg.len()).into_bytes();
        eth_message.extend_from_slice(msg.as_bytes());
        let res = keccak256(&mut eth_message).to_vec();

        //         let sig = Signature::try_from(hex!(
        //      "761c8f1b4f4d24685999cd0a6a43800cbd5fbcb5cfb83db374cb1b2d7713ced801cc4ef6ce3080bcae1db3ebdc3f5b3696c4c1dac552b7eb1e4348da7d6bad22"
        //  ).as_slice()).unwrap();

        let mut sig =hex!(
        "761c8f1b4f4d24685999cd0a6a43800cbd5fbcb5cfb83db374cb1b2d7713ced801cc4ef6ce3080bcae1db3ebdc3f5b3696c4c1dac552b7eb1e4348da7d6bad22"
        ).as_slice().to_vec();

        let recid_8 = 1_u8;
        sig.push(recid_8);
        // let signature = Signature::from_slice(&sig1).unwrap();
        // let recid = RecoveryId::try_from(recid_8).unwrap();
        sigs.push(sig);

        let signed_claim = SignedClaim {
            claim: complete_claim_data,
            signatures: sigs,
            message: res,
        };
        // dbg!(&signed_claim);
        let verify_proof_msg = ProofMsg {
            claim_info: claim_info,
            signed_claim: signed_claim,
        };
        let execute_msg = ExecuteMsg::VerifyProof(verify_proof_msg);
        let res = execute(deps.as_mut(), mock_env(), info, execute_msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
