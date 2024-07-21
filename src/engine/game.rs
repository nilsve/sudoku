use crate::engine::board::Board;
use crate::engine::board_generator::{BoardGenerator, BoardGeneratorError};

use rand::prelude::*;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Game {
    pub history: Vec<Board>,
    pub current: Board,
    pub solved: Board,
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Couldn't generate a new board. Amount of tries exceeded")]
    TriesExceeded,
    #[error("Couldn't generate a new board. {0}")]
    BoardGeneratorError(#[from] BoardGeneratorError),
}

pub type GameResult<T> = Result<T, GameError>;

impl Game {
    pub fn new_random(max_tries: usize, desired_cells_given: usize) -> GameResult<Game> {
        for i in 0..max_tries {
            if let Ok((solved, emptied)) = match BoardGenerator::new(thread_rng()).new_board(desired_cells_given) {
                Ok(board) => Ok(board),
                Err(BoardGeneratorError::NoDeletionsAvailable(board)) => Err(BoardGeneratorError::NoDeletionsAvailable(board)),
                Err(err) => return Err(err.into()),
            }{
                return Ok(Game {
                    history: vec![],
                    current: emptied,
                    solved: solved,
                })
            }
        }

        Err(GameError::TriesExceeded)
    }
}