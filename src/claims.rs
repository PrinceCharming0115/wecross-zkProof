use crate::ContractError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[cfg(feature = "vanilla")]
use cosmwasm_std::{Addr, CanonicalAddr, DepsMut, Uint128};

#[cfg(feature = "secret")]
use secret_std::{Addr, CanonicalAddr, DepsMut, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClaimInfo {
    pub provider: String,
    pub parameters: String,
    pub context: String,
}

impl ClaimInfo {
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        let hash_str = format!(
            "{}\n{}\n{}",
            &self.provider, &self.parameters, &self.context
        );
        hasher.update(hash_str.as_bytes().to_vec());
        let result = hasher.finalize().to_vec();
        return result;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CompleteClaimData {
    pub identifier: Vec<u8>,
    pub owner: Addr,
    pub epoch: Uint128,
    pub timestamp_s: u64,
}

impl CompleteClaimData {
    pub fn serialise(&self) -> Vec<u8> {
        let hash_str = format!(
            "{}\n{}\n{}\n{}",
            hex::encode(&self.identifier),
            &self.owner.to_string(),
            &self.timestamp_s.to_string(),
            &self.epoch.to_string()
        );
        hash_str.as_bytes().to_vec()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SignedClaim {
    pub claim: CompleteClaimData,
    pub bytes: Vec<[u8; 1]>,
}

impl SignedClaim {
    pub fn recover_signers_of_signed_claim(
        self,
        deps: DepsMut,
    ) -> Result<Vec<Addr>, ContractError> {
        // Create empty array
        let mut expected = vec![];
        // Hash the signature
        let mut hasher = Sha256::new();
        let serialised_claim = self.claim.serialise();
        hasher.update(serialised_claim);
        let result = hasher.finalize().to_vec();

        // For each signature in the claim
        for signature in self.bytes {
            // Recover the public key
            let pubkey = deps.api.secp256k1_recover_pubkey(&result, &signature, 0);
            match pubkey {
                Ok(key) => {
                    // Convert public key to human readable addr
                    let canonical_addr = CanonicalAddr::from(key);
                    let addr = deps.api.addr_humanize(&canonical_addr)?;
                    expected.push(addr)
                }
                // optimise: better error enums
                Err(..) => return Err(ContractError::PubKeyErr {}),
            }
        }
        Ok(expected)
    }
}
