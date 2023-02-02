use crate::*;

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct CastlingRight {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl CastlingRight {
    pub fn lose_white_kingside_castling_right(&mut self) {
        self.white_kingside = false;
    }
    pub fn lose_black_kingside_castling_right(&mut self) {
        self.black_kingside = false;
    }
    pub fn lose_white_queenside_castling_right(&mut self) {
        self.white_queenside = false;
    }
    pub fn lose_black_queenside_castling_right(&mut self) {
        self.black_queenside = false;
    }

    pub fn lose_all_right(&mut self, color: Color) {
        if color.is_white() {
            self.white_kingside = false;
            self.white_queenside = false;
        } else {
            self.black_kingside = false;
            self.black_queenside = false;
        }
    }

    pub fn has_kingside_right(&self, color: Color) -> bool {
        if color.is_white() {
            self.white_kingside
        } else {
            self.black_kingside
        }
    }

    pub fn has_queenside_right(&self, color: Color) -> bool {
        if color.is_white() {
            self.white_queenside
        } else {
            self.black_queenside
        }
    }

    pub fn has_right(&self, color: Color) -> bool {
        if color.is_white() {
            self.white_kingside || self.white_queenside
        } else {
            self.black_kingside || self.black_queenside
        }
    }

    pub fn update_castling_right(&mut self, color: Color, from: Square, to: Square) {
        // king moved
        if from.is_king_square(color) {
            return self.lose_all_right(color);
        }

        // rook moved
        if from.is_uppermost_rank_square() {
            if from.is_leftmost_file_square() {
                return self.lose_black_queenside_castling_right();
            } else if from.is_rightmost_file_square() {
                return self.lose_black_kingside_castling_right();
            }
        } else if from.is_lowermost_rank_square() {
            if from.is_leftmost_file_square() {
                return self.lose_white_queenside_castling_right();
            }
            if from.is_rightmost_file_square() {
                return self.lose_white_kingside_castling_right();
            }
        }

        // rook eaten
        if to.is_uppermost_rank_square() {
            if to.is_leftmost_file_square() {
                return self.lose_black_queenside_castling_right();
            } else if to.is_rightmost_file_square() {
                return self.lose_black_kingside_castling_right();
            }
        } else if to.is_lowermost_rank_square() {
            if to.is_leftmost_file_square() {
                return self.lose_white_queenside_castling_right();
            }
            if to.is_rightmost_file_square() {
                return self.lose_white_kingside_castling_right();
            }
        }
    }
}

impl Default for CastlingRight {
    fn default() -> Self {
        Self {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}
