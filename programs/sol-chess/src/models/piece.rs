use crate::*;

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Piece {
    Empty,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
}

impl Piece {
    pub fn is_black(&self) -> bool {
        match self {
            Piece::WhitePawn
            | Piece::WhiteRook
            | Piece::WhiteKnight
            | Piece::WhiteBishop
            | Piece::WhiteQueen
            | Piece::WhiteKing => false,
            Piece::BlackPawn
            | Piece::BlackRook
            | Piece::BlackKnight
            | Piece::BlackBishop
            | Piece::BlackQueen
            | Piece::BlackKing => true,
            Piece::Empty => panic!("Empty has no Color"),
        }
    }

    pub fn is_white(&self) -> bool {
        match self {
            Piece::WhitePawn
            | Piece::WhiteRook
            | Piece::WhiteKnight
            | Piece::WhiteBishop
            | Piece::WhiteQueen
            | Piece::WhiteKing => true,
            Piece::BlackPawn
            | Piece::BlackRook
            | Piece::BlackKnight
            | Piece::BlackBishop
            | Piece::BlackQueen
            | Piece::BlackKing => false,
            Piece::Empty => panic!("Empty has no Color"),
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Piece::WhitePawn
            | Piece::WhiteRook
            | Piece::WhiteKnight
            | Piece::WhiteBishop
            | Piece::WhiteQueen
            | Piece::WhiteKing => Color::White,
            Piece::BlackPawn
            | Piece::BlackRook
            | Piece::BlackKnight
            | Piece::BlackBishop
            | Piece::BlackQueen
            | Piece::BlackKing => Color::Black,
            Piece::Empty => panic!("Empty has no Color"),
        }
    }

    pub fn is_empty(&self) -> bool {
        if self == &Piece::Empty {
            return true;
        }
        return false;
    }

    pub fn is_not_empty(&self) -> bool {
        if self != &Piece::Empty {
            return true;
        }
        return false;
    }
}

impl Default for Piece {
    fn default() -> Self {
        Piece::Empty
    }
}
