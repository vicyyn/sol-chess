use crate::*;

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn is_white(&self) -> bool {
        self == &Color::White
    }

    pub fn is_black(&self) -> bool {
        self == &Color::Black
    }

    pub fn get_pawn_direction(&self) -> i8 {
        if self.is_white() {
            return -1;
        }
        return 1;
    }

    pub fn is_opposite(&self, color: Color) -> bool {
        self != &color
    }

    pub fn get_opposite(&self) -> Color {
        if self.is_white() {
            return Color::Black;
        } else {
            return Color::White;
        }
    }

    pub fn is_color(&self, color: Color) -> bool {
        self == &color
    }

    pub fn get_starting_pawn_rank(&self) -> usize {
        if self.is_white() {
            return 6;
        }
        return 1;
    }

    pub fn get_queen(&self) -> Piece {
        if self.is_white() {
            Piece::WhiteQueen
        } else {
            Piece::BlackQueen
        }
    }
}
