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

    pub fn get_upper_squares_empty(&self, square: Square) -> Vec<Square> {
        let mut squares = vec![];
        let upper_squares = square.get_upper_squares();
        for square in upper_squares {
            if self.get_piece(square).is_empty() {
                squares.push(square)
            } else {
                break;
            }
        }
        return squares;
    }

    pub fn get_lower_squares_empty(&self, square: Square) -> Vec<Square> {
        let mut squares = vec![];
        let lower_squares = square.get_lower_squares();
        for square in lower_squares {
            if self.get_piece(square).is_empty() {
                squares.push(square)
            } else {
                break;
            }
        }
        return squares;
    }

    pub fn get_right_squares_empty(&self, square: Square) -> Vec<Square> {
        let mut squares = vec![];
        let right_squares = square.get_right_squares();
        for square in right_squares {
            if self.get_piece(square).is_empty() {
                squares.push(square)
            } else {
                break;
            }
        }
        return squares;
    }

    pub fn get_left_squares_empty(&self, square: Square) -> Vec<Square> {
        let mut squares = vec![];
        let left_squares = square.get_left_squares();
        for square in left_squares {
            if self.get_piece(square).is_empty() {
                squares.push(square)
            } else {
                break;
            }
        }
        return squares;
    }

    pub fn get_upper_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let upper_squares = square.get_upper_squares();
        for square in upper_squares {
            let piece = self.get_piece(square);
            if piece.is_not_empty() {
                return Some((piece, square));
            }
        }
        return None;
    }

    pub fn get_lower_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let lower_squares = square.get_lower_squares();
        for square in lower_squares {
            let piece = self.get_piece(square);
            if piece.is_not_empty() {
                return Some((piece, square));
            }
        }
        return None;
    }

    pub fn get_right_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let right_squares = square.get_right_squares();
        for square in right_squares {
            let piece = self.get_piece(square);
            if piece.is_not_empty() {
                return Some((piece, square));
            }
        }
        return None;
    }

    pub fn get_left_piece(&self, square: Square) -> Option<(Piece, Square)> {
        let left_squares = square.get_left_squares();
        for square in left_squares {
            let piece = self.get_piece(square);
            if piece.is_not_empty() {
                return Some((piece, square));
            }
        }
        return None;
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
