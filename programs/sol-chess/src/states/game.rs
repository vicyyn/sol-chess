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

    pub fn is_valid_move(&self, color: Color, from: Square, to: Square) -> bool {
        let valid_moves = match self.board.get_piece(from) {
            Piece::WhitePawn | Piece::BlackPawn => self.get_valid_pawn_moves(color, from),
            Piece::WhiteRook | Piece::BlackRook => self.get_valid_rook_moves(color, from),
            Piece::WhiteKnight | Piece::BlackKnight => self.get_valid_knight_moves(color, from),
            Piece::WhiteBishop | Piece::BlackBishop => self.get_valid_bishop_moves(color, from),
            Piece::WhiteQueen | Piece::BlackQueen => self.get_valid_queen_moves(color, from),
            Piece::WhiteKing | Piece::BlackKing => self.get_valid_king_moves(color, from),
            _ => vec![],
        };

        if valid_moves.contains(&to) {
            return true;
        }
        return false;
    }

    pub fn not_in_check(&self, color: Color) -> bool {
        return !self.in_check(color);
    }

    pub fn in_check(&self, color: Color) -> bool {
        let king_square = self.board.get_king(color).unwrap();

        let pawn_attack_squares = king_square.get_pawn_attack_squares(color);
        let adjacent_squares = king_square.get_adjacent_squares();
        let diagonal_pieces = self.board.get_diagonal_pieces(king_square);
        let parallel_pieces = self.board.get_parallel_pieces(king_square);
        let knight_jump_pieces = self.board.get_knight_jump_pieces(king_square);

        // pawn
        for pawn_attack_square in pawn_attack_squares {
            let piece = self.board.get_piece(pawn_attack_square);
            if piece.is_pawn() && piece.get_color().is_opposite(color) {
                return true;
            }
        }

        // king
        for adjacent_square in adjacent_squares {
            let piece = self.board.get_piece(adjacent_square);
            if piece.is_king() && piece.get_color().is_opposite(color) {
                return true;
            }
        }

        // bishop / queen
        for diagonal_piece in diagonal_pieces {
            if diagonal_piece.0.get_color().is_opposite(color)
                && (diagonal_piece.0.is_bishop() || diagonal_piece.0.is_queen())
            {
                return true;
            }
        }

        // rook / queen
        for parallel_piece in parallel_pieces {
            if parallel_piece.0.get_color().is_opposite(color)
                && (parallel_piece.0.is_rook() || parallel_piece.0.is_queen())
            {
                return true;
            }
        }

        // knight
        for knight_jump_piece in knight_jump_pieces {
            if knight_jump_piece.0.get_color().is_opposite(color) && knight_jump_piece.0.is_knight()
            {
                return true;
            }
        }

        return false;
    }

    pub fn get_valid_pawn_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        // pawn steps
        let forward_square = square.get_square_forward(color);
        if self.board.get_piece(forward_square).is_empty() {
            // one step
            valid_squares.push(forward_square);

            let double_forward_square = square.get_square_double_forward(color);
            // double step
            if self.board.get_piece(double_forward_square).is_empty()
                && square.is_starting_pawn_square(color)
            {
                valid_squares.push(double_forward_square);
            }
        }

        // pawn eats
        let pawn_attack_squares = square.get_pawn_attack_squares(color);
        for pawn_attack_square in pawn_attack_squares {
            let piece = self.board.get_piece(pawn_attack_square);
            if (piece.is_not_empty() && piece.get_color().is_opposite(color))
                || (self.enpassant.is_some() && pawn_attack_square == self.enpassant.unwrap())
            {
                valid_squares.push(pawn_attack_square);
            }
        }

        return valid_squares;
    }

    pub fn get_valid_rook_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        // move
        valid_squares.extend(self.board.get_upper_squares_empty(square));
        valid_squares.extend(self.board.get_lower_squares_empty(square));
        valid_squares.extend(self.board.get_right_squares_empty(square));
        valid_squares.extend(self.board.get_left_squares_empty(square));

        // eat
        let parallel_pieces = self.board.get_parallel_pieces(square);
        for parallel_piece in parallel_pieces {
            if parallel_piece.0.get_color().is_opposite(color) {
                valid_squares.push(parallel_piece.1);
            }
        }

        return valid_squares;
    }

    pub fn get_valid_knight_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        for jump in square.get_knight_jumps() {
            let piece = self.board.get_piece(jump);
            if piece.is_empty() || piece.get_color().is_opposite(color) {
                valid_squares.push(jump);
            }
        }

        return valid_squares;
    }

    pub fn get_valid_bishop_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        // move
        valid_squares.extend(self.board.get_upper_right_squares_empty(square));
        valid_squares.extend(self.board.get_lower_right_squares_empty(square));
        valid_squares.extend(self.board.get_upper_left_squares_empty(square));
        valid_squares.extend(self.board.get_lower_left_squares_empty(square));

        // eat
        let diagonal_pieces = self.board.get_diagonal_pieces(square);
        for diagonal_piece in diagonal_pieces {
            if diagonal_piece.0.get_color().is_opposite(color) {
                valid_squares.push(diagonal_piece.1);
            }
        }

        return valid_squares;
    }

    pub fn get_valid_queen_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        valid_squares.extend(self.get_valid_rook_moves(color, square));
        valid_squares.extend(self.get_valid_bishop_moves(color, square));

        return valid_squares;
    }

    pub fn get_valid_king_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        for adjacent_square in square.get_adjacent_squares() {
            let piece = self.board.get_piece(adjacent_square);
            if piece.is_empty() || piece.get_color().is_opposite(color) {
                valid_squares.push(adjacent_square);
            }
        }

        return valid_squares;
    }

    pub fn move_piece(&mut self, color: Color, from: Square, to: Square) {
        let current_enpassant = self.enpassant.clone();
        self.reset_enpassant();

        match self.board.get_piece(from) {
            Piece::WhitePawn | Piece::BlackPawn => {
                // passant eat
                if self.enpassant.is_some() {}
                if current_enpassant.is_some() && to == current_enpassant.unwrap() {
                    self.board.eat_piece(to.get_square_backward(color))
                }
                // double step
                else if to.is_double_forward(color, from) {
                    self.set_enpassant(from.get_square_forward(color));
                }
            }
            _ => {}
        };

        self.board.move_piece(from, to);
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

    pub fn set_enpassant(&mut self, square: Square) {
        self.enpassant = Some(square);
    }

    pub fn reset_enpassant(&mut self) {
        self.enpassant = None;
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
