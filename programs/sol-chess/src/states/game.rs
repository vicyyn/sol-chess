use crate::*;

pub const SEED_GAME: &[u8] = b"game";

#[account]
pub struct Game {
    pub board: Board,
    pub state: GameState,
    pub white: Option<Pubkey>,
    pub black: Option<Pubkey>,
    pub enpassant: Option<Square>,
}

impl Game {
    pub fn pda(payer: Pubkey, game_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[SEED_GAME, payer.as_ref(), &game_id.to_be_bytes()],
            &crate::ID,
        )
    }

    pub fn is_valid_move(&mut self, from: Square, to: Square) -> bool {
        return match self.board.get_piece(from) {
            Piece::WhitePawn | Piece::BlackPawn => self.is_valid_pawn_move(from, to),
            _ => false,
        };
    }

    pub fn is_valid_pawn_move(&mut self, from: Square, to: Square) -> bool {
        let color = self.get_current_player_color();

        // pawn steps
        if self.board.get_piece(to).is_empty() {
            // one step
            if from.get_square_forward(color) == to {
                return true;
            }

            // double step
            if self
                .board
                .get_piece(from.get_square_forward(color))
                .is_empty()
                && from.get_square_double_forward(color) == to
                && from.is_starting_pawn_square(color)
            {
                // set enpassant
                self.enpassant = Some(from.get_square_forward(color));
                return true;
            }
        }

        // pawn eats
        if from.get_square_forward_left(color) == to || from.get_square_forward_right(color) == to {
            // regular eats
            let piece = self.board.get_piece(to);
            if piece.is_not_empty() && piece.get_color().is_opposite(color) {
                return true;
            }

            // enpassant
            if self.enpassant.is_some() && self.enpassant.unwrap() == to {
                return true;
            }
        }

        return false;
    }

    pub fn get_current_player_pubkey(&self) -> Pubkey {
        if self.state.get_current_player_turn().is_white() {
            self.white.unwrap()
        } else {
            self.black.unwrap()
        }
    }

    pub fn get_current_player_color(&self) -> Color {
        if self.state.get_current_player_turn().is_white() {
            return Color::White;
        } else {
            return Color::Black;
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
        self.enpassant = None;
        Ok(())
    }
}
