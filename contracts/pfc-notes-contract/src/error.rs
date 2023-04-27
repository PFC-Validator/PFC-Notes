use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;

#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Ownership(#[from] OwnershipError),
    #[error("Address does not exist: %0")]
    AddressDoesNotExist(String),
}
