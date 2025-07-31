use anchor_lang::prelude::*;

#[account]
pub struct UserBalance {
    pub user: Pubkey,
    pub locked_amount: u64,
    pub bump: u8,
}

