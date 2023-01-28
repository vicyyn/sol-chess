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
