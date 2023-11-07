use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CanonicalAddr, DepsMut, RecoverPubkeyError, Uint128};
use sha2::{Digest, Sha256};

#[cw_serde]
pub struct ClaimInfo {
    pub provider: String,
    pub parameters: String,
    pub context: String,
}

impl ClaimInfo {
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        let mut hash_str = String::from(&self.provider);
        hash_str.push_str("\n");
        hash_str.push_str(&self.parameters);
        hash_str.push_str("\n");
        hash_str.push_str(&self.context);
        hasher.update(hash_str.as_bytes().to_vec());
        let result = hasher.finalize().to_vec();
        return result;
    }
}

#[cw_serde]
pub struct CompleteClaimData {
    pub identifier: Uint128,
    pub owner: Addr,
    pub epoch: Uint128,
    pub timestamp_s: u64,
}

impl CompleteClaimData {
    pub fn serialise(&self) -> Vec<u8> {
        let mut hash_str = String::from(self.identifier);
        hash_str.push_str("\n");
        hash_str.push_str(&self.owner.to_string());
        hash_str.push_str("\n");
        hash_str.push_str(&self.timestamp_s.to_string());
        hash_str.push_str("\n");
        hash_str.push_str(&self.epoch.to_string());
        hash_str.as_bytes().to_vec()
    }
}

#[cw_serde]
pub struct SignedClaim {
    pub claim: CompleteClaimData,
    pub bytes: Vec<[u8; 1]>,
}

impl SignedClaim {
    pub fn recover_signers_of_signed_claim(
        self,
        deps: DepsMut,
    ) -> Result<Vec<Addr>, RecoverPubkeyError> {
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
            let pubkey = deps.api.secp256k1_recover_pubkey(&result, &signature, 0)?;
            // Convert public key to human readable addr
            let canonical_addr = CanonicalAddr::from(pubkey);
            let addr = deps.api.addr_humanize(&canonical_addr);
            match addr {
                Ok(address) => expected.push(address),
                Err(..) => return Err(RecoverPubkeyError::InvalidHashFormat),
            }
        }
        Ok(expected)
    }
}
