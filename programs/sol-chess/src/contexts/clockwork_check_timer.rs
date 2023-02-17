use crate::*;
use clockwork_sdk::{
    state::{AccountMetaData, InstructionData, Thread, ThreadResponse},
    utils::PAYER_PUBKEY,
};

#[derive(Accounts)]
pub struct ClockworkCheckTimer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub adversary_user: Account<'info, User>,

    #[account(mut,signer,address = Thread::pubkey(game.key(),"game_thread".to_string()))]
    pub clockwork_thread: Box<Account<'info, Thread>>,

    #[account(mut,address=Game::pda(game.owner,game.id).0)]
    pub game: Account<'info, Game>,
    pub clock: Sysvar<'info, Clock>,
}
impl<'info> ClockworkCheckTimer<'info> {
    pub fn process(&mut self) -> Result<()> {
        let Self {
            user,
            game,
            adversary_user,
            clock,
            ..
        } = self;

        let color = game.get_current_player_color();

        require!(
            game.get_adversary_player(color).eq(&adversary_user.key()),
            CustomError::InvalidAdversaryUserAccount
        );

        require!(
            user.key() == game.get_current_player_pubkey(),
            CustomError::NotUsersTurn
        );

        if game.has_no_time(color, clock.unix_timestamp) {
            game.set_winner(color.get_opposite());

            if game.has_wager() {
                adversary_user.increase_balance(game.get_wager() * 2);
            }

            if game.is_rated() {
                user.lost_against(adversary_user.get_elo());
                adversary_user.won_against(user.get_elo());
            }
        }

        Ok(())
    }
}
