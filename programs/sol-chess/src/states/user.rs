use crate::*;

pub const SEED_USER: &[u8] = b"user";

#[account]
pub struct User {
    pub current_game: Option<Pubkey>,
    pub elo: u32,
    pub games: u64,
    pub balance: u64,
}

impl User {
    pub fn pda(owner: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_USER, owner.as_ref()], &crate::ID)
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

    pub fn increase_balance(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn decrease_balance(&mut self, amount: u64) {
        self.balance -= amount;
    }

    pub fn has_sufficient(&self, amount: u64) -> bool {
        if amount <= self.balance {
            return true;
        } else {
            return false;
        }
    }

    pub fn get_elo(&self) -> u32 {
        self.elo
    }

    pub fn get_expected_score(&self, adversary_elo: u32) -> f64 {
        1.0 / (1.0 + 10f64.powf((adversary_elo as f64 - self.elo as f64) / 400.0))
    }

    pub fn get_new_elo(&self, adversary_elo: u32, score: f64) -> u32 {
        self.elo + (40.0 * (score - self.get_expected_score(adversary_elo))) as u32
    }

    pub fn won_against(&mut self, adversary_elo: u32) {
        self.elo = self.get_new_elo(adversary_elo, 1.0);
    }

    pub fn draw_against(&mut self, adversary_elo: u32) {
        self.elo = self.get_new_elo(adversary_elo, 0.5);
    }

    pub fn lost_against(&mut self, adversary_elo: u32) {
        self.elo = self.get_new_elo(adversary_elo, 0.0);
    }
}

pub trait UserAccount {
    fn new(&mut self) -> Result<()>;
}

impl UserAccount for Account<'_, User> {
    fn new(&mut self) -> Result<()> {
        self.current_game = None;
        self.elo = 800;
        self.games = 0;
        self.balance = 0;
        Ok(())
    }
}
