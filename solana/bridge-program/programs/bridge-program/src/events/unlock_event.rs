use anchor_lang::prelude::*;

#[event]
pub struct UnLockEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}