use crate::*;

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(init,payer=payer,space=8 + size_of::<Game>(), seeds=[SEED_GAME,user.key().as_ref(),&user.games.to_be_bytes()], bump)]
    pub game: Account<'info, Game>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

impl<'info> InitializeGame<'info> {
    pub fn process(&mut self, wager: Option<u64>, is_rated: bool) -> Result<()> {
        let InitializeGame {
            game, user, clock, ..
        } = self;

        user.increment_games();
        game.new(wager, clock.unix_timestamp, is_rated)?;
        Ok(())
    }
}
