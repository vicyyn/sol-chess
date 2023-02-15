use crate::*;

#[derive(Clone, Copy, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct GameConfig {
    pub timer: Option<u32>,
    pub increment: Option<u32>,
    pub is_rated: bool,
    pub wager: Option<u64>,
}

impl GameConfig {
    pub fn get_timer(&self) -> u32 {
        self.timer.unwrap()
    }

    pub fn get_increment(&self) -> u32 {
        self.increment.unwrap()
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

    pub fn valid_time_control(&self) -> bool {
        self.timer.is_some() && self.increment.is_some()
    }

    pub fn get_time_control(&self) -> Option<TimeControl> {
        if self.valid_time_control() {
            Some(TimeControl::new(self.get_timer(), self.get_increment()))
        } else {
            None
        }
    }
}
