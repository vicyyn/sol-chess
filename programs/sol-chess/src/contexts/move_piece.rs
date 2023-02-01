use crate::*;

#[derive(Accounts)]
pub struct MovePiece<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub game: Account<'info, Game>,
}
impl<'info> MovePiece<'info> {
    pub fn process(&mut self, from: Square, to: Square) -> Result<()> {
        let Self { user, game, .. } = self;
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
        game.next_turn();

        Ok(())
    }
}
