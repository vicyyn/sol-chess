use crate::*;

#[derive(Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Square {
    pub rank: usize,
    pub file: usize,
}

impl Square {
    pub fn get_rank(&self) -> usize {
        return self.rank;
    }

    pub fn get_file(&self) -> usize {
        return self.file;
    }
}
