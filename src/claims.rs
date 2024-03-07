use crate::ContractError;
mod identity_digest;
#[cfg(feature = "vanilla")]
use cosmwasm_std::{Addr, CanonicalAddr, DepsMut, Uint128};
use k256::{
    ecdsa::{RecoveryId, Signature, VerifyingKey}, // type aliases
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[cfg(feature = "secret")]
use secret_std::DepsMut;

pub fn append_0x(content: &str) -> String {
    let mut initializer = String::from("0x");
    initializer.push_str(content);
    initializer
}

pub fn keccak256(message: &str) -> Vec<u8> {
    let message: &[u8] = message.as_ref();

    let mut eth_message = format!("\x19Ethereum Signed Message:\n{}", message.len()).into_bytes();
    eth_message.extend_from_slice(message);
    let mut hasher = Keccak256::new();
    hasher.update(&eth_message);

    let hash = hasher.finalize().to_vec();
    hash
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClaimInfo {
    pub provider: String,
    pub parameters: String,
    pub context: String,
}

impl ClaimInfo {
    pub fn hash(&self) -> String {
        let mut hasher = Keccak256::new();
        let hash_str = format!(
            "{}\n{}\n{}",
            &self.provider, &self.parameters, &self.context
        );
        hasher.update(&hash_str);

        let hash = hasher.finalize().to_vec();
        append_0x(hex::encode(hash).as_str())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CompleteClaimData {
    pub identifier: String,
    pub owner: String,
    pub epoch: u64,
    pub timestamp_s: u64,
}

impl CompleteClaimData {
    pub fn serialise(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}",
            &self.identifier,
            &self.owner.to_string(),
            &self.timestamp_s.to_string(),
            &self.epoch.to_string()
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CompleteSignature {
    pub signature: String,
    pub recovery_param: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SignedClaim {
    pub claim: CompleteClaimData,
    pub signatures: Vec<CompleteSignature>,
}

impl SignedClaim {
    pub fn recover_signers_of_signed_claim(
        self,
        _deps: DepsMut,
    ) -> Result<Vec<String>, ContractError> {
        use crate::claims::identity_digest::Identity256;
        use digest::Update;
        // Create empty array
        let mut expected = vec![];
        // Hash the signature
        let serialised_claim = self.claim.serialise();

        let bm = keccak256(serialised_claim.as_str());
        let message_hash = bm.to_vec();

        // For each signature in the claim
        for complete_signature in self.signatures {
            let r_s = hex::decode(complete_signature.signature).unwrap();
            let recovery_param = complete_signature.recovery_param;

            let id = match recovery_param {
                0 => RecoveryId::new(false, false),
                1 => RecoveryId::new(true, false),
                _ => return Err(ContractError::SignatureErr {}),
            };

            let signature = Signature::from_bytes(r_s.as_slice().into()).unwrap();
            let message_digest = Identity256::new().chain(&message_hash);

            // Recover the public key
            let verkey = VerifyingKey::recover_from_digest(message_digest, &signature, id).unwrap();
            let key: Vec<u8> = verkey.to_encoded_point(false).as_bytes().into();
            let hasher = Keccak256::new_with_prefix(&key[1..]);

            let hash = hasher.finalize().to_vec();

            let address_bytes = hash.get(12..).unwrap();
            let public_key = append_0x(&hex::encode(address_bytes));
            expected.push(public_key);
        }
        Ok(expected)
    }
}
