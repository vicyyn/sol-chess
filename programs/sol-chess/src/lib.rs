use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("CCdU3zmYqPZaR2twy5hqcJmFV36tRpFC81seKUE8HVwX");

pub mod contexts;
pub mod error;
pub mod models;
pub mod states;

pub use contexts::*;
pub use error::*;
pub use models::*;
pub use states::*;

#[program]
pub mod sol_chess {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn initialize_game(
        ctx: Context<InitializeGame>,
        wager: Option<u64>,
        is_rated: bool,
    ) -> Result<()> {
        ctx.accounts.process(wager, is_rated)
    }

    pub fn join_game(ctx: Context<JoinGame>, color: Color) -> Result<()> {
        ctx.accounts.process(color)
    }

    pub fn move_piece(ctx: Context<MovePiece>, from: Square, to: Square) -> Result<()> {
        ctx.accounts.process(from, to)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.process(vault_bump, amount)
    }

    pub fn leave_game(ctx: Context<LeaveGame>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn resign(ctx: Context<Resign>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn offer_draw(ctx: Context<OfferDraw>) -> Result<()> {
        ctx.accounts.process()
    }
}
