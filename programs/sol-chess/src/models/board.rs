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
