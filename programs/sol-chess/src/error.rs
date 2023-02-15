use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("User Already In Game")]
    UserAlreadyInGame,
    #[msg("Color Not Available")]
    ColorNotAvailable,
    #[msg("Invalid Game State")]
    InvalidGameState,
    #[msg("Not User's Turn")]
    NotUsersTurn,
    #[msg("Invalid Move")]
    InvalidMove,
    #[msg("King in Check")]
    KingInCheck,
    #[msg("Insufficient Balance")]
    InsufficientBalance,
    #[msg("Not In Game")]
    NotInGame,
    #[msg("Game Already Started")]
    GameAlreadyStarted,
    #[msg("Invalid Adversary User Account")]
    InvalidAdversaryUserAccount,
    #[msg("User Already In Game")]
    AlreadyInGame,
    #[msg("Already Offered Draw")]
    AlreadyOfferedDraw,
    #[msg("TimeHasRunOut")]
    TimeHasRunOut,
}
