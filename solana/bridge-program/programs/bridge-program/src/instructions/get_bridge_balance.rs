use anchor_lang::prelude::*;
use crate::records::bridge_accounts::BridgeAccount;

#[derive(Accounts)]
pub struct GetBridgeBalance<'info> {
    #[account(
        mut,
        seeds = [b"bridge_vault_v1"],
        bump = bridge_account.bump
    )]
    pub bridge_account: Account<'info, BridgeAccount>,
}

pub fn handler(ctx: Context<GetBridgeBalance>) -> Result<u64> {
    Ok(ctx.accounts.bridge_account.total_locked)
}