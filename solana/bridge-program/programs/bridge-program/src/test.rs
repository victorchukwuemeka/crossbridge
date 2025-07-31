use anchor_lang::prelude::*;

declare_id!("AVzgt6tYPgiL2xW9Ykb7vbtWQY6RC8495zEXwEBu7cSa");

pub mod instructions;
pub mod events;
pub mod errors;
pub mod records;



use crate::instructions::lock_sol::LockSol;
use crate::instructions::initialize::Initialize;
use crate::instructions::unlock_sol::UnLockSol;
use crate::instructions::get_bridge_balance::GetBridgeBalance;
use crate::instructions::get_user_balance::GetUserBalance;
use crate::instructions::update_relayer::UpdateRelayer;

#[program]
pub mod bridge_program {
    use super::*;  // This already brings in UnLockSol from above
    
    
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn lock_sol(ctx: Context<LockSol>, amount: u64, eth_address: String) -> Result<()> {
        instructions::lock_sol::handler(ctx, amount, eth_address)
    }

    pub fn un_lock_sol(ctx: Context<UnLockSol>, amount: u64) -> Result<()> {
        instructions::unlock_sol::handler(ctx, amount)
    }

    pub fn get_bridge_balance(ctx: Context<GetBridgeBalance>) -> Result<u64> {
        instructions::get_bridge_balance::handler(ctx)
    }

    pub fn get_user_balance(ctx: Context<GetUserBalance>) -> Result<u64> {
        instructions::get_user_balance::handler(ctx)
    }
}