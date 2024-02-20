use ecdsa::RecoveryId;
use k256::ecdsa::{Signature, VerifyingKey};
use keccak_hash::keccak256;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[cfg(feature = "vanilla")]
use cosmwasm_std::DepsMut;

#[cfg(feature = "secret")]
use secret_std::DepsMut;

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
    pub owner: String,
    pub epoch: u64,
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
    pub signatures: Vec<(Vec<u8>, u8)>,
}

impl SignedClaim {
    pub fn recover_signers_of_signed_claim(self, _deps: DepsMut) -> Vec<String> {
        // Create empty array
        let mut expected = vec![];
        // Hash the signature
        let mut hasher = Sha256::new();
        let serialised_claim = self.claim.serialise();
        hasher.update(serialised_claim);
        let mut result = hasher.finalize().to_vec();
        keccak256(&mut result);

        for (sig, recid_8) in self.signatures {
            let signature = Signature::from_slice(&sig).unwrap();
            let recid = RecoveryId::try_from(recid_8).unwrap();
            let recovered_key =
                VerifyingKey::recover_from_prehash(&result, &signature, recid).unwrap();

            let mut str_recovered_key =
                base16::encode_lower(&recovered_key.to_sec1_bytes()).split_off(26);
            str_recovered_key.insert_str(0, "0x");

            expected.push(str_recovered_key);
        }

        expected
    }

    pub fn recover_raw_signature(signature: String) -> [u8; 64] {
        let ss = signature.as_str();
        let sss = &ss[28..156].to_lowercase();
        let sss_str = sss.as_str();
        let mut arr = [0_u8; 64];
        for i in 0..64 {
            let ss = &sss_str[(2 * i)..(2 * i + 2)];
            let z = u8::from_str_radix(ss, 16).unwrap();
            arr[i] = z;
        }
        arr
    }
}
