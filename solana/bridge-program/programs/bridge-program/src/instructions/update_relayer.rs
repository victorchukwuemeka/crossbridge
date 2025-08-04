use anchor_lang::prelude::*;
use crate::{records::bridge_accounts::BridgeAccount, errors::ErrorCode};


#[derive(Accounts)]
pub struct UpdateRelayer<'info>{
    #[account(
        mut,
        seeds = [b"bridge_vault_v2"],
        bump = bridge_account.bump
    )]
    pub bridge_account: Account<'info,BridgeAccount>,
    
    #[account(signer)]
    ///CHECK this account validate changes the bridge publckey .
    pub authority:AccountInfo<'info>
}

pub fn handler(ctx :Context<UpdateRelayer>, new_relayer:Pubkey)->Result<()>{
    require!(ctx.accounts.authority.key() == ctx.accounts.bridge_account.relayer, ErrorCode::Unauthorized);
    ctx.accounts.bridge_account.relayer = new_relayer;
    Ok(())
}