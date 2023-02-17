use crate::*;

#[derive(Accounts)]
pub struct OfferDraw<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub adversary_user: Account<'info, User>,

    #[account(mut,address=Game::pda(game.owner,game.id).0)]
    pub game: Account<'info, Game>,
}

impl<'info> OfferDraw<'info> {
    pub fn process(&mut self) -> Result<()> {
        let Self {
            user,
            game,
            adversary_user,
            ..
        } = self;

        let color = game.get_player_color(user.key());

        require!(game.is_in_game(user.key()), CustomError::NotInGame);
        require!(game.is_still_going(), CustomError::InvalidGameState);
        require!(
            game.get_adversary_player(color).eq(&adversary_user.key()),
            CustomError::InvalidAdversaryUserAccount
        );
        require!(
            game.has_not_offered_draw(color),
            CustomError::AlreadyOfferedDraw
        );

        game.update_draw_state(color);
        if game.is_draw() {
            game.set_draw();

            if game.has_wager() {
                user.increase_balance(game.get_wager());
                adversary_user.increase_balance(game.get_wager());
            }

            if game.is_rated() {
                user.draw_against(adversary_user.get_elo());
                adversary_user.draw_against(user.get_elo());
            }
        }

        Ok(())
    }
}
