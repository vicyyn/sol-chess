use crate::*;

pub const SEED_USER: &[u8] = b"user";

#[account]
pub struct User {
    pub current_game: Option<Pubkey>,
    pub elo: u64,
    pub games: u64,
}

impl User {
    pub fn pda(user: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_USER, user.as_ref()], &crate::ID)
    }

    pub fn set_game(&mut self, game: Pubkey) {
        self.current_game = Some(game);
    }

    pub fn increment_games(&mut self) {
        self.games += 1;
    }

    pub fn in_game(&self) -> bool {
        self.current_game.is_some()
    }

    pub fn not_in_game(&self) -> bool {
        self.current_game.is_none()
    }
}

pub trait UserAccount {
    fn new(&mut self) -> Result<()>;
}

impl UserAccount for Account<'_, User> {
    fn new(&mut self) -> Result<()> {
        self.current_game = None;
        self.elo = 0;
        self.games = 0;
        Ok(())
    }
}
