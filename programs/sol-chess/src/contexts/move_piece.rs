use crate::*;
use clockwork_sdk::{
    state::{Thread, ThreadSettings, Trigger},
    ThreadProgram,
};

#[derive(Accounts)]
pub struct MovePiece<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,address=User::pda(payer.key()).0)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub adversary_user: Account<'info, User>,

    #[account(mut, address=Game::pda(game.owner,game.id).0)]
    pub game: Account<'info, Game>,
    pub clock: Sysvar<'info, Clock>,
}
impl<'info> MovePiece<'info> {
    pub fn process(&mut self, from: Square, to: Square) -> Result<()> {
        let Self {
            user,
            game,
            adversary_user,
            clock,
            payer,
        } = self;
        let color = game.get_current_player_color();

        require!(
            game.has_time(color, clock.unix_timestamp),
            CustomError::TimeHasRunOut
        );

        require!(
            user.key() == game.get_current_player_pubkey(),
            CustomError::NotUsersTurn
        );

        require!(
            game.get_adversary_player(color).eq(&adversary_user.key()),
            CustomError::InvalidAdversaryUserAccount
        );

        require!(
            game.is_valid_move(color, from, to),
            CustomError::InvalidMove
        );

        game.move_piece(color, from, to);

        require!(game.not_in_check(color), CustomError::KingInCheck);

        game.next_turn();

        game.reset_draw_state();

        if game.in_checkmate(color.get_opposite()) {
            game.set_winner(color);
            if game.has_wager() {
                user.increase_balance(game.get_wager() * 2)
            }

            if game.is_rated() {
                user.won_against(adversary_user.get_elo());
                adversary_user.lost_against(user.get_elo());
            }
        }

        game.update_time_control(color, clock.unix_timestamp);

        //if game.is_first_move() {
        //    let clockwork_check_timer = Instruction {
        //        program_id: crate::ID,
        //        accounts: vec![
        //            AccountMeta::new(user.key(), false),
        //            AccountMeta::new(adversary_user.key(), false),
        //            AccountMeta::new(game_thread.key(), true),
        //            AccountMeta::new(game.key(), false),
        //            AccountMeta::new_readonly(clock.key(), true),
        //        ],
        //        data: clockwork_sdk::utils::anchor_sighash("clockwork_check_timer").into(),
        //    };

        //    clockwork_sdk::cpi::thread_create(
        //        CpiContext::new_with_signer(
        //            thread_program.to_account_info(),
        //            clockwork_sdk::cpi::ThreadCreate {
        //                authority: game.to_account_info(),
        //                payer: payer.to_account_info(),
        //                system_program: system_program.to_account_info(),
        //                thread: game_thread.to_account_info(),
        //            },
        //            &[&[
        //                SEED_GAME,
        //                &game.owner.as_ref(),
        //                &game.id.to_be_bytes(),
        //                &[game.bump],
        //            ]],
        //        ),
        //        "game_thread".to_string(),
        //        clockwork_check_timer.into(),
        //        Trigger::Immediate {},
        //    )?;
        //}

        Ok(())
    }
}
