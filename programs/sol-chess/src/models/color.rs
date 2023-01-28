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
}
