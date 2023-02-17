use crate::*;

pub const SEED_GAME: &[u8] = b"game";

#[account]
pub struct Game {
    pub created_at: i64,
    pub owner: Pubkey,
    pub id: u64,
    pub bump: u8,

    pub board: Board,
    pub game_state: GameState,
    pub white: Option<Pubkey>,
    pub black: Option<Pubkey>,
    pub enpassant: Option<Square>,
    pub castling_right: CastlingRight,
    pub draw_state: DrawState,
    pub game_config: GameConfig,
    pub time_control: TimeControl,
}

impl Game {
    pub fn pda(payer: Pubkey, game_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[SEED_GAME, payer.as_ref(), &game_id.to_be_bytes()],
            &crate::ID,
        )
    }

    pub fn is_valid_move(&self, color: Color, from: Square, to: Square) -> bool {
        let valid_moves = self.get_piece_valid_moves(color, from);
        if valid_moves.contains(&to) {
            return true;
        }
        return false;
    }

    pub fn in_checkmate(&mut self, color: Color) -> bool {
        if self.not_in_check(color) {
            return false;
        }

        let pieces = self.board.get_color_pieces(color);
        for piece in pieces {
            let valid_moves = self.get_piece_valid_moves(color, piece.1);
            for valid_move in valid_moves {
                let eaten_piece = self.board.get_piece(valid_move);
                self.board.move_piece(piece.1, valid_move);
                if self.not_in_check(color) {
                    self.board.undo_move(piece.1, valid_move, eaten_piece);
                    return false;
                }

                self.board.undo_move(piece.1, valid_move, eaten_piece);
            }
        }

        return true;
    }

    pub fn get_piece_valid_moves(&self, color: Color, square: Square) -> Vec<Square> {
        return match self.board.get_piece(square) {
            Piece::WhitePawn | Piece::BlackPawn => self.get_valid_pawn_moves(color, square),
            Piece::WhiteRook | Piece::BlackRook => self.get_valid_rook_moves(color, square),
            Piece::WhiteKnight | Piece::BlackKnight => self.get_valid_knight_moves(color, square),
            Piece::WhiteBishop | Piece::BlackBishop => self.get_valid_bishop_moves(color, square),
            Piece::WhiteQueen | Piece::BlackQueen => self.get_valid_queen_moves(color, square),
            Piece::WhiteKing | Piece::BlackKing => self.get_valid_king_moves(color, square),
            _ => vec![],
        };
    }

    pub fn not_in_check(&self, color: Color) -> bool {
        return !self.in_check(color);
    }

    pub fn in_check(&self, color: Color) -> bool {
        let king_square = self.board.get_king(color).unwrap();
        return self.board.is_square_attacked(king_square, color);
    }

    pub fn get_valid_pawn_moves(&self, color: Color, square: Square) -> Vec<Square> {
        let mut valid_squares = vec![];

        // pawn steps
        let forward_square = square.get_square_forward(color);
        if self.board.get_piece(forward_square).is_empty() {
            // one step
            valid_squares.push(forward_square);

            // double step
            if !square.is_starting_pawn_square(color.get_opposite()) {
                let double_forward_square = square.get_square_double_forward(color);
                if self.board.get_piece(double_forward_square).is_empty()
                    && square.is_starting_pawn_square(color)
                {
                    valid_squares.push(double_forward_square);
                }
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

        // regular move
        for adjacent_square in square.get_adjacent_squares() {
            let piece = self.board.get_piece(adjacent_square);
            if piece.is_empty() || piece.get_color().is_opposite(color) {
                valid_squares.push(adjacent_square);
            }
        }

        // castling
        if self.castling_right.has_kingside_right(color) && self.board.can_kingside_castle(color) {
            valid_squares.push(Square::get_kingside_castle_king_square(color))
        }

        if self.castling_right.has_queenside_right(color) && self.board.can_queenside_castle(color)
        {
            valid_squares.push(Square::get_queenside_castle_king_square(color))
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
                // promotion
                } else if to.is_last_rank(color) {
                    self.board.set_piece(color.get_queen(), from)
                }
            }
            Piece::WhiteKing | Piece::BlackKing => {
                if from.is_king_square(color) {
                    // castle kingside
                    if to.is_kingside_castle_square(color) {
                        self.board.apply_kingside_castle_rook(color)
                    } else if to.is_queenside_castle_square(color) {
                        self.board.apply_queenside_castle_rook(color)
                    }
                }
            }
            _ => {}
        };

        if self.castling_right.has_right(color) {
            self.castling_right.update_castling_right(color, from, to);
        }
        self.board.move_piece(from, to);
    }

    pub fn get_current_player_pubkey(&self) -> Pubkey {
        if self.game_state.get_current_player_turn().is_white() {
            self.white.unwrap()
        } else {
            self.black.unwrap()
        }
    }

    pub fn get_current_player_color(&self) -> Color {
        if self.game_state.get_current_player_turn().is_white() {
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
        self.game_state.start_game();
    }

    pub fn next_turn(&mut self) {
        self.game_state = self.game_state.next_turn()
    }

    pub fn set_enpassant(&mut self, square: Square) {
        self.enpassant = Some(square);
    }

    pub fn reset_enpassant(&mut self) {
        self.enpassant = None;
    }

    pub fn set_winner(&mut self, color: Color) {
        if color.is_white() {
            self.game_state.set_white_winner()
        } else {
            self.game_state.set_black_winner()
        }
    }

    pub fn has_wager(&self) -> bool {
        self.game_config.has_wager()
    }

    pub fn get_wager(&self) -> u64 {
        self.game_config.get_wager()
    }

    pub fn is_in_game(&self, player: Pubkey) -> bool {
        self.white.eq(&Some(player)) || self.black.eq(&Some(player))
    }

    pub fn is_not_in_game(&self, player: Pubkey) -> bool {
        !self.white.eq(&Some(player)) && !self.black.eq(&Some(player))
    }

    pub fn get_player_color(&self, player: Pubkey) -> Color {
        if self.white.eq(&Some(player)) {
            return Color::White;
        } else {
            return Color::Black;
        }
    }

    pub fn leave_game(&mut self, color: Color) {
        if color.is_white() {
            self.white = None;
        } else {
            self.black = None;
        }
    }

    pub fn is_not_started(&self) -> bool {
        self.game_state.is_waiting()
    }

    pub fn is_still_going(&self) -> bool {
        self.game_state.is_still_going()
    }

    pub fn get_adversary_player(&self, color: Color) -> Pubkey {
        if color.is_white() {
            return self.black.unwrap();
        } else {
            return self.white.unwrap();
        }
    }

    pub fn is_draw(&self) -> bool {
        self.draw_state.is_draw()
    }

    pub fn update_draw_state(&mut self, color: Color) {
        self.draw_state.update_state(color);
    }

    pub fn set_draw(&mut self) {
        self.game_state.set_draw();
    }

    pub fn reset_draw_state(&mut self) {
        self.draw_state.reset();
    }

    pub fn has_not_offered_draw(&self, color: Color) -> bool {
        !self.draw_state.color_offered(color)
    }

    pub fn is_rated(&self) -> bool {
        self.game_config.is_rated()
    }

    pub fn is_first_move(&self) -> bool {
        self.time_control.is_first_move()
    }

    pub fn has_time(&self, color: Color, current_timestamp: i64) -> bool {
        self.time_control.has_time(color, current_timestamp)
    }

    pub fn has_no_time(&self, color: Color, current_timestamp: i64) -> bool {
        !self.has_time(color, current_timestamp)
    }

    pub fn update_time_control(&mut self, color: Color, current_timestamp: i64) {
        self.time_control
            .update_time_control(color, current_timestamp)
    }
}

pub trait GameAccount {
    fn new(
        &mut self,
        game_config: GameConfig,
        created_at: i64,
        owner: Pubkey,
        id: u64,
        bump: u8,
    ) -> Result<()>;
}

impl GameAccount for Account<'_, Game> {
    fn new(
        &mut self,
        game_config: GameConfig,
        created_at: i64,
        owner: Pubkey,
        id: u64,
        bump: u8,
    ) -> Result<()> {
        self.created_at = created_at;
        self.owner = owner;
        self.id = id;
        self.bump = bump;

        self.board = Board::default();
        self.game_state = GameState::Waiting;
        self.white = None;
        self.black = None;
        self.enpassant = None;
        self.castling_right = CastlingRight::default();
        self.draw_state = DrawState::Neither;
        self.game_config = game_config;
        self.time_control = game_config.get_time_control();
        Ok(())
    }
}
