use anchor_lang::prelude::*;
//use crate::accounts::bridge_accounts;
use crate::records::bridge_accounts::BridgeAccount;
use std::str::FromStr;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 + 1 + 32, // discriminator + u64 + bump
        seeds = [b"bridge_vault_v1"],
        bump
    )]
    pub bridge_account: Account<'info, BridgeAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let bridge_account = &mut ctx.accounts.bridge_account;
    bridge_account.bump = ctx.bumps.bridge_account;
    bridge_account.total_locked = 0;
    bridge_account.relayer = Pubkey::from_str("4dmQAcJe9Ksh4FtpMMfHajP4ssBhrbNrPrGc3v5jFFSA").unwrap();
    Ok(())
}