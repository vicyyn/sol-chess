use crate::*;

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub enum GameState {
    Waiting,
    White,
    Black,
    WhiteWon,
    BlackWon,
    Draw,
}

impl GameState {
    pub fn get_current_player_turn(&self) -> Color {
        match self {
            GameState::White => Color::White,
            GameState::Black => Color::Black,
            _ => panic!("Invalid Game State"),
        }
    }
}
