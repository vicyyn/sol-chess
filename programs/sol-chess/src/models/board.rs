use crate::*;

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Board([[Piece; 8]; 8]);

impl Board {
    pub fn get_piece(&self, square: Square) -> Piece {
        return self.0[square.get_rank()][square.get_file()];
    }

    pub fn set_piece(&mut self, piece: Piece, square: Square) {
        self.0[square.get_rank()][square.get_file()] = piece;
    }

    pub fn move_piece(&mut self, from: Square, to: Square) {
        let piece = self.get_piece(from);
        self.set_piece(Piece::Empty, from);
        self.set_piece(piece, to);
    }
}
