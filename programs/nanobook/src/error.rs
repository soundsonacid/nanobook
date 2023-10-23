use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The maximum number of orders has been reached.")]
    MaxOrdersReached,
    #[msg("Invalid order cancellation")]
    CouldNotCancel,
    #[msg("Could not fill order")]
    CouldNotFill,
    #[msg("Could not find order")]
    CouldNotFind,
    #[msg("Attempted to withdraw more than balance")]
    Overdraft,
    #[msg("User map full")]
    UserMapFull,
    #[msg("User not found")]
    UserNotFound,
    #[msg("User already exists")]
    UserAlreadyExists,
}