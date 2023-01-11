use anchor_lang::prelude::*;

declare_id!("3QG4u81iuHg8a7KhpsgJr2J4teJbUFs3ngtgBKG8zvA1");

#[program]
pub mod sol_chess {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
