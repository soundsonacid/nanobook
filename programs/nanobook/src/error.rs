use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The maximum number of orders has been reached.")]
    MaxOrdersReached,
    #[msg("Invalid order cancellation")]
    CouldNotCancel,
}