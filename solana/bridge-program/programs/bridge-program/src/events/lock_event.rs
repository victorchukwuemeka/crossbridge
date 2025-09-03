use anchor_lang::prelude::*;

#[event]
pub struct LockEvent {
    pub user: Pubkey,
    pub eth_address: String,
    pub amount: u64,
    pub fee: u64,
    pub target_network: u8,
    pub timestamp: i64,
}