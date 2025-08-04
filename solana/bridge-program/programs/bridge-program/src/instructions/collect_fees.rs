use anchor_lang::prelude::*;
use crate::records::bridge_accounts::BridgeAccount;


// admin pubkey constant
const ADMIN_PUBLICKEY:Pubkey = pubkey!("4dmQAcJe9Ksh4FtpMMfHajP4ssBhrbNrPrGc3v5jFFSA");


#[derive(Accounts)]
pub struct CollectFees<'info>{
    #[account(
        mut,
        seeds = [b"bridge_vault_v2"],
        bump = bridge_account.bump
    )]
    pub bridge_account : Account<'info, BridgeAccount>,

    #[account(mut)]
    pub admin : Signer<'info>,

    /// CHECK: Fee collector wallet
    #[account(mut)]
    pub fee_collector: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CollectFees>)->Result<()>{
    
    require!(
        ctx.accounts.admin.key() == ADMIN_PUBLICKEY,
        ErrorCode::Unauthorized,
    );

    let fees_to_collect = ctx.accounts.bridge_account.fees_collected;
    require!(fees_to_collect > 0, ErrorCode::NoFeesToCollect);

    **ctx.accounts.bridge_account.to_account_info().try_borrow_mut_lamports()? -= fees_to_collect;
    **ctx.accounts.fee_collector.to_account_info().try_borrow_mut_lamports()? +=  fees_to_collect;
    
    ctx.accounts.bridge_account.fees_collected = 0;

    Ok(())
}


#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("No fees to collect")]
    NoFeesToCollect,
}