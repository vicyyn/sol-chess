use crate::*;

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,

    #[account(mut,address=Game::pda(game.owner,game.id).0)]
    pub game: Account<'info, Game>,
}

impl<'info> JoinGame<'info> {
    pub fn process(&mut self, color: Color) -> Result<()> {
        let Self { user, game, .. } = self;

        require!(game.color_available(color), CustomError::ColorNotAvailable);
        // require!(game.is_not_in_game(user.key()), CustomError::AlreadyInGame);

        user.set_game(game.key());
        game.join_game(user.key(), color);

        if game.is_full() {
            game.start_game()
        }

        if game.has_wager() {
            let wager = game.get_wager();
            require!(user.has_sufficient(wager), CustomError::InsufficientBalance);
            user.decrease_balance(wager);
        }

        Ok(())
    }
}
