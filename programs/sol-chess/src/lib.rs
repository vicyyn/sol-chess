use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("3QG4u81iuHg8a7KhpsgJr2J4teJbUFs3ngtgBKG8zvA1");

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

    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn join_game(ctx: Context<JoinGame>, color: Color) -> Result<()> {
        ctx.accounts.process(color)
    }

    pub fn move_piece(ctx: Context<MovePiece>, from: Square, to: Square) -> Result<()> {
        ctx.accounts.process(from, to)
    }
}
