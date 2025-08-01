// lib.rs
use anchor_lang::prelude::*;

declare_id!("7N9UCyKUqac5JuEjn4inZcBFhi87FXDRy3rP1mNhTrdB");

// Import your modules
pub mod instructions;
pub mod events;
pub mod errors;
pub mod records;

// Re-export everything so the #[program] macro can see it
pub use instructions::*;
pub use events::*;
pub use errors::*;
pub use records ::*;


#[program]
pub mod bridge_program {
    use super::*;

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