use crate::*;

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub game: Account<'info, Game>,
}
impl<'info> JoinGame<'info> {
    pub fn process(&mut self, color: Color) -> Result<()> {
        let Self { user, game, .. } = self;

        require!(game.color_available(color), CustomError::ColorNotAvailable);
        // require!(user.not_in_game(), CustomError::UserAlreadyInGame);

        user.set_game(game.key());
        game.join_game(user.key(), color);
        Ok(())
    }
}
