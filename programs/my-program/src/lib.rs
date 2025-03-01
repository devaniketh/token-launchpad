use anchor_lang::prelude::*;

declare_id!("8GE2C6ysvJcYnNicjJdySFw7uS4KvaQhAhkrdM6EPaML");

#[program]
pub mod my_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
