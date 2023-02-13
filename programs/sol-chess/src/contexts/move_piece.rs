use crate::*;

#[derive(Accounts)]
pub struct MovePiece<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub adversary_user: Account<'info, User>,

    #[account(mut)]
    pub game: Account<'info, Game>,
}
impl<'info> MovePiece<'info> {
    pub fn process(&mut self, from: Square, to: Square) -> Result<()> {
        let Self {
            user,
            game,
            adversary_user,
            ..
        } = self;
        let color = game.get_current_player_color();

        require!(
            user.key() == game.get_current_player_pubkey(),
            CustomError::NotUsersTurn
        );

        require!(
            game.is_valid_move(color, from, to),
            CustomError::InvalidMove
        );

        game.move_piece(color, from, to);

        require!(game.not_in_check(color), CustomError::KingInCheck);

        game.next_turn();

        if game.in_checkmate(color.get_opposite()) {
            game.set_winner(color);
            if game.has_wager() {
                user.increase_balance(game.get_wager() * 2)
            }

            user.won_against(adversary_user.get_elo());
            adversary_user.lost_against(user.get_elo());
        }

        game.reset_draw_state();

        Ok(())
    }
}
