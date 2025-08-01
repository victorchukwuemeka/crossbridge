use anchor_lang::prelude::*;
use crate::{records::{bridge_accounts::BridgeAccount, user_balance::UserBalance}, events::lock_event::LockEvent};

#[derive(Accounts)]
pub struct LockSol<'info> {
    #[account(
        mut,
        seeds = [b"bridge_vault_v1"],
        bump = bridge_account.bump
    )]
    pub bridge_account: Account<'info, BridgeAccount>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 1, // discriminator + pubkey + u64 + bump
        seeds = [b"user_balance", user.key().as_ref()],
        bump
    )]
    pub user_balance: Account<'info, UserBalance>,


    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<LockSol>, amount: u64, eth_address: String) -> Result<()> {
    let transfer = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user.key(),
        &ctx.accounts.bridge_account.key(),
        amount,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.bridge_account.to_account_info(),
        ]
    )?;

    ctx.accounts.bridge_account.total_locked += amount;
    ctx.accounts.user_balance.locked_amount += amount;

    ctx.accounts.user_balance.bump = ctx.bumps.user_balance;

    emit!(LockEvent {
        user: ctx.accounts.user.key(),
        eth_address,
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}