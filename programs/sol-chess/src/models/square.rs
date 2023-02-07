use crate::*;
use std::{collections::HashSet, iter::FromIterator};

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq, Hash, Eq)]
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
            rank: self.rank - 1,
            file: self.file + 1,
        };
    }

    pub fn get_square_up_left(&self) -> Square {
        return Square {
            rank: self.rank - 1,
            file: self.file - 1,
        };
    }

    pub fn get_square_down_right(&self) -> Square {
        return Square {
            rank: self.rank + 1,
            file: self.file + 1,
        };
    }

    pub fn get_square_down_left(&self) -> Square {
        return Square {
            rank: self.rank + 1,
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

    pub fn is_before_leftmost_file_square(&self) -> bool {
        if self.file == 1 {
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

    pub fn is_before_rightmost_file_square(&self) -> bool {
        if self.file == 6 {
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

    pub fn get_knight_upper_jumps(&self) -> Vec<Square> {
        if self.is_starting_pawn_square(Color::Black) || self.is_uppermost_rank_square() {
            return vec![];
        } else if self.is_leftmost_file_square() {
            let upper_right_jump = self.get_square_up().get_square_up().get_square_right();
            return vec![upper_right_jump];
        } else if self.is_rightmost_file_square() {
            let upper_left_jump = self.get_square_up().get_square_up().get_square_left();
            return vec![upper_left_jump];
        } else {
            let upper_right_jump = self.get_square_up().get_square_up().get_square_left();
            let upper_left_jump = self.get_square_up().get_square_up().get_square_right();
            return vec![upper_left_jump, upper_right_jump];
        }
    }

    pub fn get_knight_lower_jumps(&self) -> Vec<Square> {
        if self.is_starting_pawn_square(Color::White) || self.is_lowermost_rank_square() {
            return vec![];
        } else if self.is_leftmost_file_square() {
            let lower_right_jump = self.get_square_down().get_square_down().get_square_right();
            return vec![lower_right_jump];
        } else if self.is_rightmost_file_square() {
            let lower_left_jump = self.get_square_down().get_square_down().get_square_left();
            return vec![lower_left_jump];
        } else {
            let lower_right_jump = self.get_square_down().get_square_down().get_square_left();
            let lower_left_jump = self.get_square_down().get_square_down().get_square_right();
            return vec![lower_left_jump, lower_right_jump];
        }
    }

    pub fn get_knight_right_jumps(&self) -> Vec<Square> {
        if self.is_rightmost_file_square() || self.is_before_rightmost_file_square() {
            return vec![];
        } else if self.is_uppermost_rank_square() {
            let right_lower_jump = self.get_square_right().get_square_right().get_square_down();
            return vec![right_lower_jump];
        } else if self.is_lowermost_rank_square() {
            let right_upper_jump = self.get_square_right().get_square_right().get_square_up();
            return vec![right_upper_jump];
        } else {
            let right_lower_jump = self.get_square_right().get_square_right().get_square_down();
            let right_upper_jump = self.get_square_right().get_square_right().get_square_up();
            return vec![right_upper_jump, right_lower_jump];
        }
    }

    pub fn get_knight_left_jumps(&self) -> Vec<Square> {
        if self.is_leftmost_file_square() || self.is_before_leftmost_file_square() {
            return vec![];
        } else if self.is_uppermost_rank_square() {
            let left_lower_jump = self.get_square_left().get_square_left().get_square_down();
            return vec![left_lower_jump];
        } else if self.is_lowermost_rank_square() {
            let left_upper_jump = self.get_square_left().get_square_left().get_square_up();
            return vec![left_upper_jump];
        } else {
            let left_lower_jump = self.get_square_left().get_square_left().get_square_down();
            let left_upper_jump = self.get_square_left().get_square_left().get_square_up();
            return vec![left_upper_jump, left_lower_jump];
        }
    }

    pub fn get_knight_jumps(&self) -> Vec<Square> {
        let mut squares = vec![];
        squares.extend(self.get_knight_upper_jumps());
        squares.extend(self.get_knight_lower_jumps());
        squares.extend(self.get_knight_right_jumps());
        squares.extend(self.get_knight_left_jumps());
        return squares;
    }

    pub fn get_upper_right_squares(&self) -> Vec<Square> {
        let mut squares = vec![];
        let mut current_square = self.clone();

        while !current_square.is_uppermost_rank_square()
            && !current_square.is_rightmost_file_square()
        {
            let upper_right_square = current_square.get_square_up().get_square_right();
            squares.push(upper_right_square);
            current_square = upper_right_square.clone();
        }

        return squares;
    }

    pub fn get_lower_right_squares(&self) -> Vec<Square> {
        let mut squares = vec![];
        let mut current_square = self.clone();

        while !current_square.is_lowermost_rank_square()
            && !current_square.is_rightmost_file_square()
        {
            let lower_right_square = current_square.get_square_down().get_square_right();
            squares.push(lower_right_square);
            current_square = lower_right_square.clone();
        }

        return squares;
    }

    pub fn get_upper_left_squares(&self) -> Vec<Square> {
        let mut squares = vec![];
        let mut current_square = self.clone();

        while !current_square.is_uppermost_rank_square()
            && !current_square.is_leftmost_file_square()
        {
            let upper_left_square = current_square.get_square_up().get_square_left();
            squares.push(upper_left_square);
            current_square = upper_left_square.clone();
        }

        return squares;
    }

    pub fn get_lower_left_squares(&self) -> Vec<Square> {
        let mut squares = vec![];
        let mut current_square = self.clone();

        while !current_square.is_lowermost_rank_square()
            && !current_square.is_leftmost_file_square()
        {
            let lower_left_square = current_square.get_square_down().get_square_left();
            squares.push(lower_left_square);
            current_square = lower_left_square.clone();
        }

        return squares;
    }

    pub fn get_upper_adjacent_squares(&self) -> Vec<Square> {
        if self.is_uppermost_rank_square() {
            return vec![];
        } else if self.is_rightmost_file_square() {
            return vec![self.get_square_up(), self.get_square_up_left()];
        } else if self.is_leftmost_file_square() {
            return vec![self.get_square_up(), self.get_square_up_right()];
        } else {
            return vec![
                self.get_square_up(),
                self.get_square_up_right(),
                self.get_square_up_left(),
            ];
        }
    }

    pub fn get_lower_adjacent_squares(&self) -> Vec<Square> {
        if self.is_lowermost_rank_square() {
            return vec![];
        } else if self.is_rightmost_file_square() {
            return vec![self.get_square_down(), self.get_square_down_left()];
        } else if self.is_leftmost_file_square() {
            return vec![self.get_square_down(), self.get_square_down_right()];
        } else {
            return vec![
                self.get_square_down(),
                self.get_square_down_right(),
                self.get_square_down_left(),
            ];
        }
    }

    pub fn get_right_adjacent_squares(&self) -> Vec<Square> {
        if self.is_rightmost_file_square() {
            return vec![];
        } else if self.is_uppermost_rank_square() {
            return vec![self.get_square_right(), self.get_square_down_right()];
        } else if self.is_lowermost_rank_square() {
            return vec![self.get_square_right(), self.get_square_up_right()];
        } else {
            return vec![
                self.get_square_right(),
                self.get_square_up_right(),
                self.get_square_down_right(),
            ];
        }
    }

    pub fn get_left_adjacent_squares(&self) -> Vec<Square> {
        if self.is_leftmost_file_square() {
            return vec![];
        } else if self.is_uppermost_rank_square() {
            return vec![self.get_square_left(), self.get_square_down_left()];
        } else if self.is_lowermost_rank_square() {
            return vec![self.get_square_left(), self.get_square_up_left()];
        } else {
            return vec![
                self.get_square_left(),
                self.get_square_up_left(),
                self.get_square_down_left(),
            ];
        }
    }

    pub fn get_adjacent_squares(&self) -> Vec<Square> {
        let mut squares: HashSet<Square> = HashSet::new();

        squares.extend::<HashSet<Square>>(HashSet::from_iter(self.get_upper_adjacent_squares()));
        squares.extend::<HashSet<Square>>(
            HashSet::from_iter(self.get_lower_adjacent_squares()) as HashSet<Square>
        );
        squares.extend::<HashSet<Square>>(
            HashSet::from_iter(self.get_right_adjacent_squares()) as HashSet<Square>
        );
        squares.extend::<HashSet<Square>>(
            HashSet::from_iter(self.get_left_adjacent_squares()) as HashSet<Square>
        );

        return Vec::from_iter(squares);
    }

    pub fn get_pawn_attack_squares(&self, color: Color) -> Vec<Square> {
        let mut squares = vec![];

        if !self.is_leftmost_file_square_relative(color) {
            let left_forward_square = self.get_square_forward_left(color);
            squares.push(left_forward_square);
        }

        if !self.is_rightmost_file_square_relative(color) {
            let right_forward_square = self.get_square_forward_right(color);
            squares.push(right_forward_square);
        }

        return squares;
    }

    pub fn is_king_square(&self, color: Color) -> bool {
        if color.is_white() {
            return self == &Square { rank: 7, file: 4 };
        } else {
            return self == &Square { rank: 0, file: 4 };
        }
    }

    pub fn is_kingside_castle_square(&self, color: Color) -> bool {
        if color.is_white() {
            self == &Square { rank: 7, file: 6 }
        } else {
            self == &Square { rank: 0, file: 6 }
        }
    }

    pub fn is_queenside_castle_square(&self, color: Color) -> bool {
        if color.is_white() {
            self == &Square { rank: 7, file: 2 }
        } else {
            self == &Square { rank: 0, file: 2 }
        }
    }

    pub fn get_kingside_castle_squares(color: Color) -> Vec<Square> {
        if color.is_white() {
            return vec![Square { rank: 7, file: 5 }, Square { rank: 7, file: 6 }];
        } else {
            return vec![Square { rank: 0, file: 5 }, Square { rank: 0, file: 6 }];
        }
    }

    pub fn get_queenside_castle_squares(color: Color) -> Vec<Square> {
        if color.is_white() {
            return vec![
                Square { rank: 7, file: 1 },
                Square { rank: 7, file: 2 },
                Square { rank: 7, file: 3 },
            ];
        } else {
            return vec![
                Square { rank: 0, file: 1 },
                Square { rank: 0, file: 2 },
                Square { rank: 0, file: 3 },
            ];
        }
    }

    pub fn get_queenside_castle_king_square(color: Color) -> Square {
        if color.is_white() {
            Square { rank: 7, file: 2 }
        } else {
            Square { rank: 0, file: 2 }
        }
    }

    pub fn get_kingside_castle_king_square(color: Color) -> Square {
        if color.is_white() {
            Square { rank: 7, file: 6 }
        } else {
            Square { rank: 0, file: 6 }
        }
    }

    pub fn is_last_rank(&self, color: Color) -> bool {
        if color.is_white() {
            self.is_uppermost_rank_square()
        } else {
            self.is_lowermost_rank_square()
        }
    }
}
