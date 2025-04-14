use anchor_lang::prelude::*;

declare_id!("28AQpwDXyQPTkcuJweUQFfAMqTkDZfNME71Anic7o5rM");

#[program]
pub mod bridge_program {
    use anchor_lang::solana_program::system_instruction;

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


    pub fn un_lock_sol(ctx: Context<UnLockSol>, amount:u64)-> Result<()>{
        let bridge_program =  &mut ctx.accounts.bridge_account;
        let user_account  = &ctx.accounts.user;
        let locked_sol = bridge_program.total_locked;

        if  locked_sol < amount  {
            return Err(Error::from(ErrorCode::InsufficientFunds));
        }

        let transfer_instructions =system_instruction::transfer(
            &bridge_program.key(), 
            &user_account.key(),
            amount
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_instructions,
            &[
                bridge_program.to_account_info(),
                user_account.to_account_info(),
            ]
        )?;

        bridge_program.total_locked = bridge_program.total_locked - amount;

        //emit this event so it will be relayed
        emit!( UnLockEvent{
            user : user_account.key(),
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

#[derive(Accounts)]
pub struct UnLockSol<'info>{
    #[account(
        mut,
        seeds = [b"bridge_vault"],
        bump = bridge_account.bump
    )]
    pub bridge_account : Account<'info, BridgeAccount>,
    pub user: Signer<'info>,
    pub system_program: Program<'info,System>, 
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

#[event]
pub struct UnLockEvent{
    pub user :Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode{
    #[msg("not enough sol locked")]
    InsufficientFunds,
}