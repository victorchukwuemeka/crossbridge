use anchor_lang::prelude::*;
use crate::records::user_balance::UserBalance;

#[derive(Accounts)]
pub struct GetUserBalance<'info> {
    #[account(
        seeds = [b"user_balance", user.key().as_ref()],
        bump = user_balance.bump
    )]
    pub user_balance: Account<'info, UserBalance>,
    
    /// CHECK: This account is used as a seed for the user_balance PDA derivation
    pub user: AccountInfo<'info>,
}

pub fn handler(ctx: Context<GetUserBalance>) -> Result<u64> {
    Ok(ctx.accounts.user_balance.locked_amount)
}