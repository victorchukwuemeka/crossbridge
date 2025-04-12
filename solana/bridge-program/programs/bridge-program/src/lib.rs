use anchor_lang::prelude::*;

declare_id!("28AQpwDXyQPTkcuJweUQFfAMqTkDZfNME71Anic7o5rM");

#[program]
pub mod bridge_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let bridge_account = &mut ctx.accounts.bridge_account;
        bridge_account.bump = ctx.bumps.bridge_account;
        bridge_account.total_locked = 0;
        Ok(())
    }

    pub fn lock_sol(ctx: Context<LockSol>, amount:u64) -> Result<()>{
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

        //emit this event so it will be relayed
        emit!(LockEvent{
            user :ctx.accounts.user.key(),
            amount,
            timestamp:Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 + 1, // discriminator + u64 + bump
        seeds = [b"bridge_vault"],
        bump
    )]
    pub bridge_account: Account<'info, BridgeAccount>,
  
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LockSol<'info>{
    #[account(
        mut,
        seeds = [b"bridge_vault"],
        bump = bridge_account.bump
    )]
    pub bridge_account: Account<'info, BridgeAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,

}




#[account]
pub struct BridgeAccount{
    pub bump: u8,
    pub total_locked: u64,
}


#[event]
pub struct LockEvent {
    pub user :Pubkey,
    pub amount :u64,
    pub timestamp : i64,
}
