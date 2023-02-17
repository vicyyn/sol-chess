use crate::*;

#[derive(Accounts)]
pub struct LeaveGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(mut,address=Game::pda(game.owner,game.id).0)]
    pub game: Account<'info, Game>,
}

impl<'info> LeaveGame<'info> {
    pub fn process(&mut self) -> Result<()> {
        let Self { user, game, .. } = self;

        require!(game.is_not_started(), CustomError::GameAlreadyStarted);
        require!(game.is_in_game(user.key()), CustomError::NotInGame);

        let color = game.get_player_color(user.key());
        game.leave_game(color);

        if game.has_wager() {
            user.increase_balance(game.get_wager())
        }

        Ok(())
    }
}
