use crate::*;

#[derive(Clone, Copy, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct GameConfig {
    pub timer: u32,
    pub increment: u32,
    pub is_rated: bool,
    pub wager: Option<u64>,
}

impl GameConfig {
    pub fn get_timer(&self) -> u32 {
        self.timer
    }

    pub fn get_increment(&self) -> u32 {
        self.increment
    }

    pub fn get_wager(&self) -> u64 {
        self.wager.unwrap()
    }

    pub fn has_wager(&self) -> bool {
        self.wager.is_some()
    }

    pub fn is_rated(&self) -> bool {
        self.is_rated
    }

    pub fn get_time_control(&self) -> TimeControl {
        TimeControl::new(self.get_timer(), self.get_increment())
    }
}
