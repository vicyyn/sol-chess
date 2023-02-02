use crate::*;
use std::cmp::{max, min};

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub struct Square {
    pub rank: u8,
    pub file: u8,
}

impl Square {
    pub fn get_rank(&self) -> usize {
        return self.rank as usize;
    }

    pub fn get_file(&self) -> usize {
        return self.file as usize;
    }

    pub fn next_rank(&self) -> usize {
        return self.rank as usize + 1;
    }

    pub fn previous_rank(&self) -> usize {
        return self.rank as usize - 1;
    }

    pub fn next_file(&self) -> usize {
        return self.file as usize + 1;
    }

    pub fn previous_file(&self) -> usize {
        return self.file as usize - 1;
    }

    pub fn get_square_up(&self) -> Square {
        return Square {
            rank: self.rank - 1,
            file: self.file,
        };
    }

    pub fn get_square_down(&self) -> Square {
        return Square {
            rank: self.rank + 1,
            file: self.file,
        };
    }

    pub fn get_square_right(&self) -> Square {
        return Square {
            rank: self.rank,
            file: self.file + 1,
        };
    }

    pub fn get_square_left(&self) -> Square {
        return Square {
            rank: self.rank,
            file: self.file - 1,
        };
    }

    pub fn get_square_up_right(&self) -> Square {
        return Square {
            rank: self.rank + 1,
            file: self.file + 1,
        };
    }

    pub fn get_square_up_left(&self) -> Square {
        return Square {
            rank: self.rank + 1,
            file: self.file - 1,
        };
    }

    pub fn get_square_down_right(&self) -> Square {
        return Square {
            rank: self.rank - 1,
            file: self.file + 1,
        };
    }

    pub fn get_square_down_left(&self) -> Square {
        return Square {
            rank: self.rank - 1,
            file: self.file - 1,
        };
    }

    pub fn get_square_forward(&self, color: Color) -> Square {
        if color.is_white() {
            return self.get_square_up();
        } else {
            return self.get_square_down();
        }
    }

    pub fn get_square_backward(&self, color: Color) -> Square {
        if color.is_white() {
            return self.get_square_down();
        } else {
            return self.get_square_up();
        }
    }

    pub fn get_square_forward_right(&self, color: Color) -> Square {
        if color.is_white() {
            return self.get_square_up().get_square_right();
        } else {
            return self.get_square_down().get_square_left();
        }
    }

    pub fn get_square_forward_left(&self, color: Color) -> Square {
        if color.is_white() {
            return self.get_square_up().get_square_left();
        } else {
            return self.get_square_down().get_square_right();
        }
    }

    pub fn get_square_backward_right(&self, color: Color) -> Square {
        if color.is_white() {
            return self.get_square_down().get_square_right();
        } else {
            return self.get_square_up().get_square_left();
        }
    }

    pub fn get_square_backward_left(&self, color: Color) -> Square {
        if color.is_white() {
            return self.get_square_down().get_square_left();
        } else {
            return self.get_square_up().get_square_right();
        }
    }

    pub fn get_square_double_forward(&self, color: Color) -> Square {
        if color.is_white() {
            return self.get_square_up().get_square_up();
        } else {
            return self.get_square_down().get_square_down();
        }
    }

    pub fn is_starting_pawn_square(&self, color: Color) -> bool {
        if self.get_rank() == color.get_starting_pawn_rank() {
            return true;
        }
        return false;
    }

    pub fn is_uppermost_rank_square(&self) -> bool {
        if self.rank == 0 {
            return true;
        }
        return false;
    }

    pub fn is_lowermost_rank_square(&self) -> bool {
        if self.rank == 7 {
            return true;
        }
        return false;
    }

    pub fn is_leftmost_file_square(&self) -> bool {
        if self.file == 0 {
            return true;
        }
        return false;
    }

    pub fn is_rightmost_file_square(&self) -> bool {
        if self.file == 7 {
            return true;
        }
        return false;
    }

    pub fn is_leftmost_file_square_relative(&self, color: Color) -> bool {
        if color.is_white() {
            return self.is_leftmost_file_square();
        }
        return self.is_rightmost_file_square();
    }

    pub fn is_rightmost_file_square_relative(&self, color: Color) -> bool {
        if color.is_white() {
            return self.is_rightmost_file_square();
        }
        return self.is_leftmost_file_square();
    }

    pub fn is_double_forward(&self, color: Color, from: Square) -> bool {
        if color.get_starting_pawn_rank() == from.get_rank()
            && &from.get_square_double_forward(color) == self
        {
            return true;
        }
        return false;
    }

    pub fn get_lower_squares(&self) -> Vec<Square> {
        let mut squares = vec![];
        for rank in (self.rank + 1)..8 {
            squares.push(Square {
                rank,
                file: self.file,
            })
        }
        return squares;
    }

    pub fn get_upper_squares(&self) -> Vec<Square> {
        let mut squares = vec![];
        for rank in (0..self.rank).rev() {
            squares.push(Square {
                rank,
                file: self.file,
            })
        }
        return squares;
    }

    pub fn get_right_squares(&self) -> Vec<Square> {
        let mut squares = vec![];
        for file in (self.file + 1)..8 {
            squares.push(Square {
                rank: self.rank,
                file,
            })
        }
        return squares;
    }

    pub fn get_left_squares(&self) -> Vec<Square> {
        let mut squares = vec![];
        for file in (0..self.file).rev() {
            squares.push(Square {
                rank: self.rank,
                file,
            })
        }
        return squares;
    }
}
