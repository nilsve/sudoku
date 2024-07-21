use std::borrow::Borrow;
use crate::engine::board::BOARD_SIZE;

pub struct SudokuHashSet {
    data: [bool; BOARD_SIZE as usize + 1],
}

impl SudokuHashSet {
    pub fn new() -> SudokuHashSet {
        SudokuHashSet {
            data: [false; BOARD_SIZE as usize + 1],
        }
    }

    pub fn insert<T: Borrow<u8>>(&mut self, num: T) {
        self.data[*num.borrow() as usize] = true;
    }

    pub fn contains<T: Borrow<u8>>(&self, num: T) -> bool {
        self.data[*num.borrow() as usize]
    }
}

impl FromIterator<u8> for SudokuHashSet {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut set = SudokuHashSet::new();
        for num in iter {
            set.insert(num);
        }
        set
    }
}