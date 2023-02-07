use crate::*;

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Board {
    pub board: [[Piece; 8]; 8],
}

impl Board {
    pub fn get_piece(&self, square: Square) -> Piece {
        return self.board[square.get_rank()][square.get_file()];
    }

    pub fn set_piece(&mut self, piece: Piece, square: Square) {
        self.board[square.get_rank()][square.get_file()] = piece;
    }

    pub fn move_piece(&mut self, from: Square, to: Square) {
        let piece = self.get_piece(from);
        self.set_piece(Piece::Empty, from);
        self.set_piece(piece, to);
    }

    pub fn eat_piece(&mut self, square: Square) {
        self.board[square.get_rank()][square.get_file()] = Piece::Empty;
    }

    pub fn find_piece(&self, piece: Piece) -> Option<Square> {
        for rank in 0..8 {
            for file in 0..8 {
                let selected_piece = self.get_piece(Square { rank, file });
                if selected_piece == piece {
                    return Some(Square { rank, file });
                }
            }
        }
        return None;
    }

    pub fn get_king(&self, color: Color) -> Option<Square> {
        if color.is_white() {
            return self.find_piece(Piece::WhiteKing);
        } else {
            return self.find_piece(Piece::BlackKing);
        }
    }

    pub fn get_empty_squares(&self, squares: Vec<Square>) -> Vec<Square> {
        let mut empty_squares = vec![];
        for square in squares {
            if self.get_piece(square).is_empty() {
                empty_squares.push(square)
            } else {
                break;
            }
        }
        return empty_squares;
    }

    pub fn get_first_non_empty_square(&self, squares: Vec<Square>) -> Option<(Piece, Square)> {
        for square in squares {
            let piece = self.get_piece(square);
            if piece.is_not_empty() {
                return Some((piece, square));
            }
        }
        return None;
    }

    pub fn get_upper_squares_empty(&self, square: Square) -> Vec<Square> {
        let upper_squares = square.get_upper_squares();
        return self.get_empty_squares(upper_squares);
    }

    pub fn get_lower_squares_empty(&self, square: Square) -> Vec<Square> {
        let lower_squares = square.get_lower_squares();
        return self.get_empty_squares(lower_squares);
    }

    pub fn get_right_squares_empty(&self, square: Square) -> Vec<Square> {
        let right_squares = square.get_right_squares();
        return self.get_empty_squares(right_squares);
    }

    pub fn get_left_squares_empty(&self, square: Square) -> Vec<Square> {
        let left_squares = square.get_left_squares();
        return self.get_empty_squares(left_squares);
    }

    pub fn get_upper_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let upper_squares = square.get_upper_squares();
        return self.get_first_non_empty_square(upper_squares);
    }

    pub fn get_lower_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let lower_squares = square.get_lower_squares();
        return self.get_first_non_empty_square(lower_squares);
    }

    pub fn get_right_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let right_squares = square.get_right_squares();
        return self.get_first_non_empty_square(right_squares);
    }

    pub fn get_left_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let left_squares = square.get_left_squares();
        return self.get_first_non_empty_square(left_squares);
    }

    pub fn get_upper_right_squares_empty(&self, square: Square) -> Vec<Square> {
        let upper_right_squares = square.get_upper_right_squares();
        return self.get_empty_squares(upper_right_squares);
    }

    pub fn get_lower_right_squares_empty(&self, square: Square) -> Vec<Square> {
        let lower_right_squares = square.get_lower_right_squares();
        return self.get_empty_squares(lower_right_squares);
    }

    pub fn get_upper_left_squares_empty(&self, square: Square) -> Vec<Square> {
        let upper_left_squares = square.get_upper_left_squares();
        return self.get_empty_squares(upper_left_squares);
    }

    pub fn get_lower_left_squares_empty(&self, square: Square) -> Vec<Square> {
        let lower_left_squares = square.get_lower_left_squares();
        return self.get_empty_squares(lower_left_squares);
    }

    pub fn get_upper_right_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let upper_right_squares = square.get_upper_right_squares();
        return self.get_first_non_empty_square(upper_right_squares);
    }

    pub fn get_upper_left_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let upper_left_squares = square.get_upper_left_squares();
        return self.get_first_non_empty_square(upper_left_squares);
    }

    pub fn get_lower_right_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let lower_right_squares = square.get_lower_right_squares();
        return self.get_first_non_empty_square(lower_right_squares);
    }

    pub fn get_lower_left_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let lower_left_squares = square.get_lower_left_squares();
        return self.get_first_non_empty_square(lower_left_squares);
    }

    pub fn get_diagonal_pieces(&self, square: Square) -> Vec<(Piece, Square)> {
        let mut pieces = vec![];

        let upper_right_piece = self.get_upper_right_piece(square);
        if upper_right_piece.is_some() {
            pieces.push(upper_right_piece.unwrap())
        }

        let upper_left_piece = self.get_upper_left_piece(square);
        if upper_left_piece.is_some() {
            pieces.push(upper_left_piece.unwrap())
        }

        let lower_right_piece = self.get_lower_right_piece(square);
        if lower_right_piece.is_some() {
            pieces.push(lower_right_piece.unwrap())
        }

        let lower_left_piece = self.get_lower_left_piece(square);
        if lower_left_piece.is_some() {
            pieces.push(lower_left_piece.unwrap())
        }

        return pieces;
    }

    pub fn get_parallel_pieces(&self, square: Square) -> Vec<(Piece, Square)> {
        let mut pieces = vec![];

        let upper_piece = self.get_upper_piece(square);
        if upper_piece.is_some() {
            pieces.push(upper_piece.unwrap())
        }

        let lower_piece = self.get_lower_piece(square);
        if lower_piece.is_some() {
            pieces.push(lower_piece.unwrap())
        }

        let right_piece = self.get_right_piece(square);
        if right_piece.is_some() {
            pieces.push(right_piece.unwrap())
        }

        let left_piece = self.get_left_piece(square);
        if left_piece.is_some() {
            pieces.push(left_piece.unwrap())
        }

        return pieces;
    }

    pub fn is_square_attacked(&self, square: Square, color: Color) -> bool {
        let pawn_attack_squares = square.get_pawn_attack_squares(color);
        let adjacent_squares = square.get_adjacent_squares();
        let diagonal_pieces = self.get_diagonal_pieces(square);
        let parallel_pieces = self.get_parallel_pieces(square);
        let knight_jump_pieces = self.get_knight_jump_pieces(square);

        // pawn
        for pawn_attack_square in pawn_attack_squares {
            let piece = self.get_piece(pawn_attack_square);
            if piece.is_pawn() && piece.get_color().is_opposite(color) {
                return true;
            }
        }

        // king
        for adjacent_square in adjacent_squares {
            let piece = self.get_piece(adjacent_square);
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

    pub fn get_knight_jump_pieces(&self, square: Square) -> Vec<(Piece, Square)> {
        let mut pieces = vec![];

        for jump in square.get_knight_jumps() {
            let piece = self.get_piece(jump);
            if piece.is_not_empty() {
                pieces.push((piece, jump))
            }
        }

        return pieces;
    }

    pub fn can_kingside_castle(&self, color: Color) -> bool {
        let kingside_castle_squares = Square::get_kingside_castle_squares(color);
        for kingside_castle_square in kingside_castle_squares {
            if self.is_square_attacked(kingside_castle_square, color)
                || self.get_piece(kingside_castle_square).is_not_empty()
            {
                return false;
            }
        }
        return true;
    }

    pub fn can_queenside_castle(&self, color: Color) -> bool {
        let queenside_castle_squares = Square::get_queenside_castle_squares(color);
        for queenside_castle_square in queenside_castle_squares {
            if self.is_square_attacked(queenside_castle_square, color)
                || self.get_piece(queenside_castle_square).is_not_empty()
            {
                return false;
            }
        }
        return true;
    }

    pub fn apply_kingside_castle_rook(&mut self, color: Color) {
        if color.is_white() {
            self.move_piece(Square { rank: 7, file: 7 }, Square { rank: 7, file: 5 })
        } else {
            self.move_piece(Square { rank: 0, file: 7 }, Square { rank: 0, file: 5 })
        }
    }

    pub fn apply_queenside_castle_rook(&mut self, color: Color) {
        if color.is_white() {
            self.move_piece(Square { rank: 7, file: 0 }, Square { rank: 7, file: 3 })
        } else {
            self.move_piece(Square { rank: 0, file: 0 }, Square { rank: 0, file: 3 })
        }
    }

    pub fn get_color_pieces(&self, color: Color) -> Vec<(Piece, Square)> {
        let mut pieces = vec![];
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square { rank, file };
                let piece = self.get_piece(square);
                if piece.is_not_empty() && piece.get_color().is_color(color) {
                    pieces.push((piece, square));
                }
            }
        }
        return pieces;
    }

    pub fn undo_move(&mut self, from: Square, to: Square, piece: Piece) {
        self.move_piece(to, from);
        self.set_piece(piece, to);
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            board: [
                [
                    Piece::BlackRook,
                    Piece::BlackKnight,
                    Piece::BlackBishop,
                    Piece::BlackQueen,
                    Piece::BlackKing,
                    Piece::BlackBishop,
                    Piece::BlackKnight,
                    Piece::BlackRook,
                ],
                [
                    Piece::BlackPawn,
                    Piece::BlackPawn,
                    Piece::BlackPawn,
                    Piece::BlackPawn,
                    Piece::BlackPawn,
                    Piece::BlackPawn,
                    Piece::BlackPawn,
                    Piece::BlackPawn,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::WhitePawn,
                    Piece::WhitePawn,
                    Piece::WhitePawn,
                    Piece::WhitePawn,
                    Piece::WhitePawn,
                    Piece::WhitePawn,
                    Piece::WhitePawn,
                    Piece::WhitePawn,
                ],
                [
                    Piece::WhiteRook,
                    Piece::WhiteKnight,
                    Piece::WhiteBishop,
                    Piece::WhiteQueen,
                    Piece::WhiteKing,
                    Piece::WhiteBishop,
                    Piece::WhiteKnight,
                    Piece::WhiteRook,
                ],
            ],
        }
    }
}
