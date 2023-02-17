use crate::*;

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq, Hash, Eq)]
pub struct TimeControl {
    pub last_move: i64,
    pub white_timer: u32,
    pub black_timer: u32,
    pub increment: u32,
}

impl TimeControl {
    pub fn new(timer: u32, increment: u32) -> Self {
        Self {
            white_timer: timer,
            black_timer: timer,
            increment,
            last_move: -1,
        }
    }

    pub fn increment_white(&mut self) {
        self.white_timer += self.increment;
    }

    pub fn increment_black(&mut self) {
        self.black_timer += self.increment;
    }

    pub fn decrease_white_timer(&mut self, time_spent: u32) {
        self.white_timer -= time_spent;
    }

    pub fn decrease_black_timer(&mut self, time_spent: u32) {
        self.black_timer -= time_spent;
    }

    pub fn get_time_passed(&self, current_timestamp: i64) -> i64 {
        current_timestamp - self.last_move
    }

    pub fn has_time(&self, color: Color, current_timestamp: i64) -> bool {
        if self.is_first_move() {
            return true;
        }
        if color.is_white() {
            current_timestamp - self.last_move < self.white_timer as i64
        } else {
            current_timestamp - self.last_move < self.black_timer as i64
        }
    }

    pub fn set_white_timer(&mut self, white_timer: u32) {
        self.white_timer = white_timer;
    }

    pub fn set_black_timer(&mut self, black_timer: u32) {
        self.black_timer = black_timer;
    }

    pub fn set_last_move(&mut self, last_move: i64) {
        self.last_move = last_move;
    }

    pub fn is_first_move(&self) -> bool {
        self.last_move == -1
    }

    pub fn is_not_first_move(&self) -> bool {
        self.last_move != -1
    }

    pub fn update_time_control(&mut self, color: Color, current_timestamp: i64) {
        if self.is_not_first_move() {
            let time_spent = (current_timestamp - self.last_move) as u32;
            if color.is_white() {
                self.decrease_white_timer(time_spent);
                self.increment_white()
            } else {
                self.decrease_black_timer(time_spent);
                self.increment_black()
            }
        }
        self.set_last_move(current_timestamp)
    }
}
