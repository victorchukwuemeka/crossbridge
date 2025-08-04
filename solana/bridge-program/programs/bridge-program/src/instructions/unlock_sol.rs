use anchor_lang::prelude::*;
use crate::{records::{user_balance::UserBalance, bridge_accounts::BridgeAccount}, errors::ErrorCode, events::unlock_event::UnLockEvent};

#[derive(Accounts)]
pub struct UnLockSol<'info> {
    #[account(
        mut,
        seeds = [b"bridge_vault_v2"],
        bump = bridge_account.bump
    )]
    pub bridge_account: Account<'info, BridgeAccount>,
    

    #[account(
        mut,
        seeds = [b"user_balance", user.key().as_ref()],
        bump = user_balance.bump
    )]
    pub user_balance: Account<'info, UserBalance>,
    
    #[account(signer)]
    /// CHECK: This account is validated against bridge_account.relayer in the handler function
    pub authority: AccountInfo<'info>,

    
    /// CHECK: User account for unlocking, doesn't need to sign
    #[account(mut)]
    pub user: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}



pub fn handler(ctx: Context<UnLockSol>, amount: u64) -> Result<()> {
    
    msg!("Relayer bytes: {:?}", ctx.accounts.bridge_account.relayer.to_bytes());
    msg!("Authority bytes: {:?}", ctx.accounts.authority.key().to_bytes());
    msg!("Expected relayer: {}", ctx.accounts.bridge_account.relayer);
    msg!("Provided authority: {}", ctx.accounts.authority.key());

    require!(
        ctx.accounts.authority.key() == ctx.accounts.bridge_account.relayer,
        ErrorCode::Unauthorized
    );

     
    require!(
        ctx.accounts.user_balance.locked_amount >= amount,
        ErrorCode::InsufficientUserFunds
    );
    
    require!(
        ctx.accounts.bridge_account.total_locked >= amount,
        ErrorCode::InsufficientFunds

    );

    **ctx.accounts.bridge_account.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;

    

    
     ctx.accounts.bridge_account.total_locked -= amount;
     ctx.accounts.user_balance.locked_amount -= amount;

    emit!(UnLockEvent {
        user: ctx.accounts.user.key(),
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}