use anchor_lang::prelude::*;
use crate::{records::{bridge_accounts::BridgeAccount, user_balance::UserBalance}, events::lock_event::LockEvent};

#[derive(Accounts)]
pub struct LockSol<'info> {
    #[account(
        mut,
        seeds = [b"bridge_vault_v2"],
        bump = bridge_account.bump
    )]
    pub bridge_account: Account<'info, BridgeAccount>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 8 + 1, // discriminator + pubkey + u64 + u64 + bump
        seeds = [b"user_balance", user.key().as_ref()],
        bump
    )]
    pub user_balance: Account<'info, UserBalance>,


    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<LockSol>, amount: u64, eth_address: String, target_network:u8) -> Result<()> {
    const FEE_BPS:u64 = 1; //fee basis points in 0.01%
    let fee = amount * FEE_BPS/10000;
    let net_amount = amount.checked_sub(fee).ok_or(error!(ErrorCode::InsufficientAmountForFee))?;
    
   
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
    

    //update bridge account 
    ctx.accounts.bridge_account.total_locked = ctx.accounts.bridge_account.total_locked
        .checked_add(net_amount).ok_or(error!(ErrorCode::Overflow))?;
    ctx.accounts.bridge_account.fees_collected = ctx.accounts.bridge_account.fees_collected
        .checked_add(fee).ok_or(error!(ErrorCode::Overflow))?;


    
    //updating the user account 
    ctx.accounts.user_balance.user = ctx.accounts.user.key();
    ctx.accounts.user_balance.locked_amount = ctx.accounts.user_balance.locked_amount
    .checked_add(net_amount).ok_or(error!(ErrorCode::Overflow))?;
    ctx.accounts.user_balance.last_locked_amount = net_amount;
    ctx.accounts.user_balance.bump = ctx.bumps.user_balance;

    emit!(LockEvent {
        user: ctx.accounts.user.key(),
        eth_address,
        amount: net_amount,
        fee,
        target_network,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}


#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient amount for fee")]
    InsufficientAmountForFee,
    #[msg("Arithmetic overflow")]
    Overflow,
}