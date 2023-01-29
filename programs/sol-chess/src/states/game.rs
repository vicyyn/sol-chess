use crate::*;

pub const SEED_GAME: &[u8] = b"game";

#[account]
pub struct Game {
    pub board: Board,
    pub state: GameState,
    pub white: Option<Pubkey>,
    pub black: Option<Pubkey>,
}

impl Game {
    pub fn pda(payer: Pubkey, game_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[SEED_GAME, payer.as_ref(), &game_id.to_be_bytes()],
            &crate::ID,
        )
    }

    pub fn get_current_player_turn(&self) -> Pubkey {
        if self.state.get_current_player_turn().is_white() {
            self.white.unwrap()
        } else {
            self.black.unwrap()
        }
    }

    pub fn move_piece(&mut self, from: Square, to: Square) {
        self.board.move_piece(from, to);
    }

    pub fn join_game(&mut self, user: Pubkey, color: Color) {
        if color.is_white() {
            self.white = Some(user);
        } else {
            self.black = Some(user);
        }
    }

    pub fn color_available(&self, color: Color) -> bool {
        if color.is_white() {
            self.white.is_none()
        } else {
            self.black.is_none()
        }
    }

    pub fn is_full(&self) -> bool {
        self.white.is_some() && self.black.is_some()
    }

    pub fn start_game(&mut self) {
        self.state = GameState::White;
    }

    pub fn next_turn(&mut self) {
        self.state = self.state.next_turn()
    }
}

pub trait GameAccount {
    fn new(&mut self) -> Result<()>;
}

impl GameAccount for Account<'_, Game> {
    fn new(&mut self) -> Result<()> {
        self.board = Board::default();
        self.state = GameState::Waiting;
        self.white = None;
        self.black = None;
        Ok(())
    }
}
