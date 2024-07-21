use std::borrow::Borrow;
use std::fmt::Debug;
use crate::engine::hashsetnum::SudokuHashSet;

#[derive(Debug, Copy, Clone)]
pub enum Tile {
    Empty,
    Filled(u8)
}

#[derive(Clone)]
pub struct Board {
    rows: [[Tile; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    columns: [[Tile; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    blocks: [[Tile; BOARD_SIZE as usize]; BOARD_SIZE as usize],
}

#[derive(Copy, Clone)]
pub struct BlockCoord;
#[derive(Copy, Clone)]
pub struct TileCoord;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

impl Coord {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn to_vec_position(&self) -> usize {
        (self.y * BOARD_SIZE + self.x) as usize
    }

    pub fn to_block_index(&self) -> usize {
        (self.y / BLOCK_SIZE * BLOCK_SIZE + self.x / BLOCK_SIZE) as usize
    }

    pub fn to_index_in_block(&self) -> usize {
        (self.y % BLOCK_SIZE * BLOCK_SIZE + self.x % BLOCK_SIZE) as usize
    }
}

pub const BOARD_SIZE: u8 = 9;
pub const BLOCK_SIZE: u8 = 3;

impl Board {
    pub fn get_tile(&self, coord: &Coord) -> &Tile {
        &self.rows[coord.y as usize][coord.x as usize]
    }

    pub fn set_tile(&self, coord: &Coord, value: Tile) -> Board {
        let mut result = self.to_owned();
        result.set_tile_in_place(coord, value);
        result
    }

    pub fn set_tile_in_place(&mut self, coord: &Coord, value: Tile) {
        self.rows[coord.y as usize][coord.x as usize] = value;
        self.columns[coord.x as usize][coord.y as usize] = value;
        self.blocks[coord.to_block_index()][coord.to_index_in_block()] = value;
    }

    pub fn get_row_for_coord(&self, coord: &Coord) -> &[Tile] {
        &self.rows[coord.y as usize]
    }

    pub fn get_column_for_coord(&self, coord: &Coord) -> &[Tile] {
        &self.columns[coord.x as usize]
    }

    pub fn get_block_for_coord(&self, coord: &Coord) -> &[Tile] {
        &self.columns[(coord.x as usize * coord.y as usize) / BOARD_SIZE as usize]
    }

    pub fn verify_board(&self) -> bool {
        // Verify rows
        for row in 0..BOARD_SIZE {
            if !Board::is_valid_tile_set(self.get_row_for_coord(&Coord::new(0, row))) {
                return false;
            }
        }

        // Verify columns
        for col in 0..BOARD_SIZE {
            if !Board::is_valid_tile_set(&self.get_column_for_coord(&Coord::new(col, 0))) {
                return false;
            }
        }

        // Verify squares
        for block in 0..BOARD_SIZE {
            if !Board::is_valid_tile_set(&self.blocks[block as usize]) {
                return false;
            }
        }
        true
    }

    pub fn is_complete(&self) -> bool {
        !self.rows.iter().any(|row| {
            row.iter().any(|tile| {
                match tile {
                    Tile::Empty => true,
                    Tile::Filled(_) => false
                }
            })
        })
    }

    pub fn is_valid_tile_set<T: Borrow<Tile>>(tiles: &[T]) -> bool {
        let mut seen_numbers = SudokuHashSet::new();

        for tile in tiles {
            match tile.borrow() {
                Tile::Empty => {},
                Tile::Filled(num) => {
                    if seen_numbers.contains(num) {
                        return false;
                    }

                    seen_numbers.insert(num);
                }
            };
        }

        true
    }

    pub fn get_filled_tile_coords(&self) -> Vec<Coord> {
        let mut filled_coords = Vec::with_capacity(BOARD_SIZE as usize * BOARD_SIZE as usize);
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let coord = Coord::new(x, y);

                match self.get_tile(&coord) {
                    Tile::Empty => {},
                    Tile::Filled(_) => {
                        filled_coords.push(coord);
                    }
                }
            }
        }

        filled_coords
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\n")?;
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                match self.get_tile(&Coord::new(col, row)) {
                    Tile::Empty => write!(f, " .")?,
                    Tile::Filled(value) => write!(f, " {}", value)?
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            columns: [[Tile::Empty; BOARD_SIZE as usize]; BOARD_SIZE as usize],
            rows: [[Tile::Empty; BOARD_SIZE as usize]; BOARD_SIZE as usize],
            blocks: [[Tile::Empty; BOARD_SIZE as usize]; BOARD_SIZE as usize],
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_tile_set_valid() {
        let tiles = vec![
            Tile::Filled(1), Tile::Filled(2), Tile::Filled(3),
            Tile::Filled(4), Tile::Filled(5), Tile::Filled(6),
            Tile::Filled(7), Tile::Filled(8), Tile::Filled(9)
        ];
        assert!(Board::is_valid_tile_set(&tiles));
    }

    #[test]
    fn test_verify_tile_set_invalid() {
        let tiles = vec![
            Tile::Filled(1), Tile::Filled(2), Tile::Filled(3),
            Tile::Filled(4), Tile::Filled(5), Tile::Filled(6),
            Tile::Filled(7), Tile::Filled(9), Tile::Filled(9)
        ];
        assert!(!Board::is_valid_tile_set(&tiles));
    }

    #[test]
    fn coord_to_block_index() {
        let coord = Coord::new(3, 3);
        assert_eq!(coord.to_block_index(), 4);

        let coord = Coord::new(2, 3);
        assert_eq!(coord.to_block_index(), 3);
    }

    #[test]
    fn coord_to_index_in_block() {
        let coord = Coord::new(3, 3);
        assert_eq!(coord.to_index_in_block(), 0);

        let coord = Coord::new(2, 3);
        assert_eq!(coord.to_index_in_block(), 2);
    }
}