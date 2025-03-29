use anchor_lang::prelude::*;

declare_id!("28AQpwDXyQPTkcuJweUQFfAMqTkDZfNME71Anic7o5rM");

#[program]
pub mod bridge_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
