use crate::*;

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq, Hash, Eq)]
pub struct TimeControl {
    last_move: i64,
    white_timer: i64,
    black_timer: i64,
    increment: i64,
}

impl TimeControl {
    pub fn increment_white(&mut self) {
        self.white_timer += self.increment;
    }

    pub fn increment_black(&mut self) {
        self.black_timer += self.increment;
    }

    pub fn get_time_passed(&self, current_timestamp: i64) -> i64 {
        current_timestamp - self.last_move
    }

    pub fn still_has_time(&self, color: Color, current_timestamp: i64) -> bool {
        if color.is_white() {
            current_timestamp < self.white_timer
        } else {
            current_timestamp < self.black_timer
        }
    }
}
