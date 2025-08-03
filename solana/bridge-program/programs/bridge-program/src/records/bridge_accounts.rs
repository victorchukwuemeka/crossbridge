use anchor_lang::prelude::*;

#[account]
pub struct BridgeAccount {
    pub bump: u8,
    pub total_locked: u64,
    pub relayer: Pubkey,
    pub fees_collected: u64
}