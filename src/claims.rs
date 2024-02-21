use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use web3::signing::{hash_message, recover};

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
        let hash_str = format!(
            "{}\n{}\n{}",
            &self.provider, &self.parameters, &self.context
        );
        let bm = hash_message(hash_str);
        let message = bm.as_bytes().to_vec();

        return message;
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
    pub signatures: Vec<Vec<u8>>,
    pub message: Vec<u8>,
}

impl SignedClaim {
    pub fn recover_signers_of_signed_claim(self, _deps: DepsMut) -> Vec<String> {
        let mut expected = vec![];

        for sig in self.signatures {
            let pubkey = recover(&self.message, &sig[..64], sig[65].into());
            let pubkey = pubkey.unwrap();
            let pubkey = format!("{:02X?}", pubkey);

            expected.push(pubkey);
        }

        expected
    }
}
