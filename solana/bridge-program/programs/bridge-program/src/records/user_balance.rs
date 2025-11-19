use anchor_lang::prelude::*;

#[account]
pub struct UserBalance {
    pub user: Pubkey,
    pub locked_amount: u64,
    pub last_locked_amount: u64,
    pub bump: u8,

    //when i added the field for the privacy
    pub commitment: [u8; 32],
    pub nullifier: [u8; 32]
    
}
