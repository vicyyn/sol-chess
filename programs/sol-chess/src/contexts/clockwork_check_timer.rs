use crate::*;
use clockwork_sdk::state::Thread;

#[derive(Accounts)]
pub struct ClockworkCheckTimer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub adversary_user: Account<'info, User>,

    #[account(mut,signer,address = Thread::pubkey(game.key(),"game_thread".to_string()))]
    pub game_thread: Box<Account<'info, Thread>>,

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

        require!(game.is_in_game(user.key()), CustomError::NotInGame);
        require!(
            game.is_in_game(game.get_adversary_player(color)),
            CustomError::NotInGame
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
