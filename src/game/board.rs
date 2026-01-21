use std::ops::{Index, IndexMut};

/// A 10x20 Tetris board.
///
/// Coordinate system:
/// - `board[0]` is the **bottom** row
/// - `board[19]` is the **top** row
/// - `board[row][0]` is the **left** column
/// - `board[row][9]` is the **right** column
///
/// Supports indexing: `board[row][col]` or `board[row]` for a full row.
pub struct Board {
    cells: [[bool; 10]; 20],
}

impl Index<usize> for Board {
    type Output = [bool; 10];

    fn index(&self, row: usize) -> &Self::Output {
        &self.cells[row]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.cells[row]
    }
}

impl Board {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 20;

    /// Creates a new empty board.
    pub fn new() -> Self {
        Self {
            cells: [[false; Self::WIDTH]; Self::HEIGHT],
        }
    }

    /// Creates a board from a cell array.
    pub fn from_cells(cells: [[bool; 10]; 20]) -> Self {
        Self { cells }
    }

    /// Returns the height of a column (number of rows from bottom to highest block).
    /// Returns 0 if the column is empty.
    pub fn column_height(&self, col: usize) -> usize {
        for row in (0..Self::HEIGHT).rev() {
            if self[row][col] {
                return row + 1;
            }
        }
        0
    }

    /// Returns the row index of the highest occupied cell (from bottom).
    /// Returns None if the board is empty.
    pub fn highest_occupied_row(&self) -> Option<usize> {
        for row in (0..Self::HEIGHT).rev() {
            if self[row].iter().any(|&c| c) {
                return Some(row);
            }
        }
        None
    }

    /// Iterates rows from bottom to top.
    pub fn rows_bottom_up(&self) -> impl Iterator<Item = (usize, &[bool; 10])> {
        self.cells.iter().enumerate()
    }

    /// Iterates rows from top to bottom. (0 is the top row)
    pub fn rows_top_down(&self) -> impl Iterator<Item = (usize, &[bool; 10])> {
        self.cells.iter().rev().enumerate()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
