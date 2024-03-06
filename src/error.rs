use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    #[error("EPOCH id already exists")]
    AlreadyExists {},

    #[error("Key recovery error")]
    PubKeyErr {},
    #[error("Signature not appropriate")]
    SignatureErr {},
    #[error("Hash mismatch")]
    HashMismatchErr {},
    #[error("Not enough witness")]
    WitnessMismatchErr {},
    #[error("Cannot find")]
    NotFoundErr {},
}