use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("not enough sol locked")]
    InsufficientFunds,
    #[msg("Unauthorized relayer")]
    Unauthorized,
    #[msg("Insufficient user funds")]
    InsufficientUserFunds,
}