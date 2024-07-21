use std::borrow::Borrow;
use rand::prelude::SliceRandom;
use rand::{RngCore};
use thiserror::Error;
use crate::engine::board::{Board, BOARD_SIZE, Coord, Tile};
use crate::engine::hashsetnum::SudokuHashSet;

#[derive(Error, Debug)]
pub enum BoardGeneratorError {
    #[error("No number available")]
    NoNumberAvailable,
    #[error("Multiple solutions available")]
    MultipleSolutionsAvailable,
    #[error("No more tiles to delete")]
    NoDeletionsAvailable(Board)
}

pub type BoardGeneratorResult<T> = Result<T, BoardGeneratorError>;

pub struct BoardGenerator<Rng: RngCore> {
    rng: Rng,
}

impl<Rng: RngCore> BoardGenerator<Rng> {
    pub fn new(rng: Rng) -> BoardGenerator<Rng> {
        BoardGenerator {
            rng
        }
    }

    pub fn get_numbers_for_tiles<T: Borrow<Tile>>(tiles: &[T]) -> SudokuHashSet {
        tiles.iter().filter_map(|tile| {
            match tile.borrow() {
                Tile::Empty => None,
                Tile::Filled(val) => Some(*val)
            }
        }).collect()
    }

    fn update_board_with_random_number(&mut self, board: &Board, coord: &Coord, excluding: &SudokuHashSet) -> BoardGeneratorResult<(u8, Board)> {
        let mut nums: Vec<u8> = (1..BOARD_SIZE + 1).filter(|num| {
            !excluding.contains(num)
        }).collect();

        nums.shuffle(&mut self.rng);

        let mut new_board = board.clone();

        for num in nums {
            new_board.set_tile_in_place(&coord, Tile::Filled(num));

            if new_board.verify_board() {
                return Ok((num, new_board));
            }
        }

        Err(BoardGeneratorError::NoNumberAvailable)
    }

    pub fn new_board(&mut self, desired_cells_given: usize) -> BoardGeneratorResult<(Board, Board)> {
        let solved_board = self.try_fill_board(Board::default()).unwrap();

        let emptied_board = self.try_empty_board(solved_board.clone(), desired_cells_given, Vec::new())?;

        Ok((solved_board, emptied_board))
    }

    fn try_empty_board(&mut self, mut board: Board, desired_cells_given: usize, mut unreplacable_coords: Vec<Coord>) -> BoardGeneratorResult<Board> {
        let mut filled_coords: Vec<Coord> = board.get_filled_tile_coords().into_iter().filter(|coord| {
           !unreplacable_coords.contains(coord)
        }).collect();

        filled_coords.shuffle(&mut self.rng);

        // Try to replace the number with something else, see if it's valid, and see if we can still fill the board

        let mut has_replaced_a_tile = false;
        for random_tile_coord in filled_coords {
            let Tile::Filled(original_value) = board.get_tile(&random_tile_coord) else {
                panic!("Empty tile received");
            };

            let is_replacable_by_something_else = {
                let mut has_options = false;
                for i in (1..BOARD_SIZE + 1).filter(|num| *num != *original_value) {
                    let updated_board = board.set_tile(&random_tile_coord, Tile::Filled(i));

                    if updated_board.verify_board() {
                        match self.try_fill_board(updated_board) {
                            Ok(_) => {
                                unreplacable_coords.push(random_tile_coord.clone());
                                has_options = true;
                            },
                            Err(BoardGeneratorError::MultipleSolutionsAvailable) => {
                                unreplacable_coords.push(random_tile_coord.clone());
                                has_options = true;
                            },
                            Err(BoardGeneratorError::NoNumberAvailable) => {},
                            Err(err) => return Err(err)
                        };
                    }
                }

                has_options
            };

            if !is_replacable_by_something_else {
                board = board.set_tile(&random_tile_coord, Tile::Empty);
                has_replaced_a_tile = true;

                break;
            }
        }

        if !has_replaced_a_tile {
            return Err(BoardGeneratorError::NoDeletionsAvailable(board))
        }

        if board.get_filled_tile_coords().len() == desired_cells_given {
            return Ok(board);
        }

        return self.try_empty_board(board, desired_cells_given, unreplacable_coords);
    }

    pub fn try_fill_board(&mut self, board: Board) -> BoardGeneratorResult<Board> {
        for x in (0..BOARD_SIZE).collect::<Vec<u8>>() {
            for y in (0..BOARD_SIZE).collect::<Vec<u8>>() {
                let coord = Coord::new(x, y);

                if let Tile::Empty = board.get_tile(&coord) {
                    let mut excluding_numbers = SudokuHashSet::new();
                    loop {
                        let (num, board) = self.update_board_with_random_number(&board, &coord, &excluding_numbers)?;

                        excluding_numbers.insert(num);

                        match self.try_fill_board(board) {
                            Ok(board) => {
                                return Ok(board);
                            }
                            Err(BoardGeneratorError::NoNumberAvailable) => {},
                            Err(err) => return Err(err)
                        };
                    }
                }
            }
        }

        if board.is_complete() {
            return Ok(board)
        }

        panic!("Shouldn't be reachable");
    }
}