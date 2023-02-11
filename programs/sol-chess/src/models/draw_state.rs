use crate::*;

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum DrawState {
    Neither,
    White,
    Black,
    Draw,
}

impl DrawState {
    pub fn color_offered(&self, color: Color) -> bool {
        if color.is_white() {
            self == &DrawState::White
        } else {
            self == &DrawState::Black
        }
    }

    pub fn set_color(&mut self, color: Color) {
        if color.is_white() {
            *self = DrawState::White;
        } else {
            *self = DrawState::Black;
        }
    }

    pub fn white_offered(&self) -> bool {
        self == &DrawState::White
    }

    pub fn black_offered(&self) -> bool {
        self == &DrawState::Black
    }

    pub fn is_draw_with(&self, color: Color) -> bool {
        color.is_white() && self.black_offered() || color.is_black() && self.white_offered()
    }

    pub fn is_draw(&self) -> bool {
        self == &DrawState::Draw
    }

    pub fn is_neither(&self) -> bool {
        self == &DrawState::Neither
    }

    pub fn reset(&mut self) {
        *self = DrawState::Neither;
    }

    pub fn set_white(&mut self) {
        *self = DrawState::White;
    }

    pub fn set_black(&mut self) {
        *self = DrawState::White;
    }

    pub fn set_draw(&mut self) {
        *self = DrawState::Draw;
    }

    pub fn update_state(&mut self, color: Color) {
        if color.is_white() && self.black_offered() {
            self.set_draw();
        } else if color.is_black() && self.white_offered() {
            self.set_draw();
        } else if self.is_neither() {
            self.set_color(color);
        }
    }
}
