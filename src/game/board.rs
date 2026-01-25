use std::fmt::{self, Display, Write};
use std::ops::{Index, IndexMut};

use super::tetromino::FallingPiece;

/// A 10x20 Tetris board.
///
/// Coordinate system:
/// - `board[0]` is the **bottom** row
/// - `board[19]` is the **top** row
/// - `board[row][0]` is the **left** column
/// - `board[row][9]` is the **right** column
///
/// Supports indexing: `board[row][col]` or `board[row]` for a full row.
#[derive(Clone)]
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
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cells: [[false; Self::WIDTH]; Self::HEIGHT],
        }
    }

    /// Creates a board from a cell array.
    #[must_use]
    pub const fn from_cells(cells: [[bool; 10]; 20]) -> Self {
        Self { cells }
    }

    /// Returns the height of a column (number of rows from bottom to highest block).
    /// Returns 0 if the column is empty.
    #[must_use]
    pub fn column_height(&self, col: usize) -> usize {
        for row in (0..Self::HEIGHT).rev() {
            if self.cells[row][col] {
                return row + 1;
            }
        }
        0
    }

    /// Iterates rows from bottom to top.
    pub fn rows_bottom_up(&self) -> impl Iterator<Item = (usize, &[bool; 10])> {
        self.cells.iter().enumerate()
    }

    /// Iterates rows from top to bottom. (0 is the top row)
    pub fn rows_top_down(&self) -> impl Iterator<Item = (usize, &[bool; 10])> {
        self.cells.iter().rev().enumerate()
    }

    /// Returns an iterator over all cell positions (col, row).
    pub fn all_positions() -> impl Iterator<Item = (usize, usize)> {
        (0..Self::WIDTH).flat_map(|col| (0..Self::HEIGHT).map(move |row| (col, row)))
    }

    /// Returns an iterator with all cells flattened (occupied: true).
    pub fn all_cells(&self) -> impl Iterator<Item = &bool> {
        self.cells.iter().flat_map(|row| row.iter())
    }

    /// Checks if a cell position is within board bounds.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn in_bounds(col: i8, row: i8) -> bool {
        col >= 0 && col < Self::WIDTH as i8 && row >= 0 && row < Self::HEIGHT as i8
    }

    /// Checks if a cell position is occupied (out of bounds counts as occupied).
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub const fn is_occupied(&self, col: i8, row: i8) -> bool {
        if !Self::in_bounds(col, row) {
            return true;
        }
        self.cells[row as usize][col as usize]
    }

    /// Checks if a piece can be placed at its current position.
    /// Returns true if all cells are within bounds and unoccupied.
    #[must_use]
    pub fn can_place(&self, piece: &FallingPiece) -> bool {
        piece
            .cells()
            .iter()
            .all(|&(col, row)| !self.is_occupied(col, row))
    }

    pub fn can_lock(&self, piece: &FallingPiece) -> bool {
        let cells = piece.cells();

        // 1. Check for collisions (Original Logic)
        // The piece must fit entirely into empty space.
        let no_collision = self.can_place(&piece);

        if !no_collision {
            return false;
        }
        // To be a valid "placement" (resting spot), at least one cell of the piece
        // must be sitting on top of something solid (the floor or another block).
        let is_grounded = cells.iter().any(|&(col, row)| {
            // Check if we are on the floor (row 0) OR if the cell below is occupied
            row == 0 || self.is_occupied(col, row - 1)
        });

        is_grounded
    }

    /// Places a piece on the board, filling the cells.
    /// Panics if the piece cannot be placed (use `can_place` first).
    #[allow(clippy::cast_sign_loss)]
    pub fn place(&mut self, piece: &FallingPiece) {
        for (col, row) in piece.cells() {
            debug_assert!(
                Self::in_bounds(col, row),
                "Piece cell out of bounds: ({col}, {row})",
            );
            self.cells[row as usize][col as usize] = true;
        }
    }

    /// Returns a new board with the piece placed.
    /// Panics if the piece cannot be placed.
    #[must_use]
    pub fn with_piece(&self, piece: &FallingPiece) -> Self {
        let mut new_board = self.clone();
        new_board.place(piece);
        new_board
    }

    /// Checks if a row is completely filled.
    #[must_use]
    pub fn is_row_full(&self, row: usize) -> bool {
        self.cells[row].iter().all(|&c| c)
    }

    /// Returns indices of all full rows (bottom to top order).
    #[must_use]
    pub fn full_rows(&self) -> Vec<usize> {
        (0..Self::HEIGHT).filter(|&r| self.is_row_full(r)).collect()
    }

    /// Clears full rows and returns the number of rows cleared.
    /// Rows above cleared rows drop down.
    #[allow(clippy::cast_possible_truncation)]
    pub fn clear_full_rows(&mut self) -> u32 {
        let full = self.full_rows();
        let count = full.len() as u32;

        if count == 0 {
            return 0;
        }

        // Clear rows from top to bottom to simplify shifting
        for &row in full.iter().rev() {
            self.remove_row(row);
        }

        count
    }

    /// Removes a single row and shifts all rows above it down.
    fn remove_row(&mut self, row: usize) {
        for r in row..Self::HEIGHT - 1 {
            self.cells[r] = self.cells[r + 1];
        }
        // Clear the top row
        self.cells[Self::HEIGHT - 1] = [false; Self::WIDTH];
    }

    /// Drops a piece down as far as possible (hard drop).
    /// Returns the piece at its final position, or None if it can't be placed at all.
    #[must_use]
    pub fn hard_drop(&self, piece: &FallingPiece) -> Option<FallingPiece> {
        if !self.can_place(piece) {
            return None;
        }

        let mut dropped = *piece;
        while self.can_place(&dropped.moved(0, -1)) {
            dropped = dropped.moved(0, -1);
        }
        Some(dropped)
    }

    /// Returns the number of rows the piece would drop.
    #[must_use]
    pub fn drop_distance(&self, piece: &FallingPiece) -> u32 {
        let mut distance = 0;
        let mut test_piece = *piece;
        while self.can_place(&test_piece.moved(0, -1)) {
            test_piece = test_piece.moved(0, -1);
            distance += 1;
        }
        distance
    }

    /// Counts total occupied cells on the board.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn cell_count(&self) -> u32 {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c)
            .count() as u32
    }

    /// Checks if the board is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|&c| !c))
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(clippy::cast_possible_truncation)]
        let cells = self
            .cells
            .iter()
            .enumerate()
            .flat_map(|(row, cols)| {
                cols.iter()
                    .enumerate()
                    .filter(|&(_, occupied)| *occupied)
                    .map(move |(col, _)| (col as i8, row as i8))
            })
            .collect::<Vec<_>>();

        visualize_cells(f, &cells, Self::WIDTH, Self::HEIGHT)
    }
}

/// Renders a set of cells as a text grid.
///
/// Cells are rendered as `█`, empty spaces as `.`.
/// Grid is displayed top-to-bottom (highest row first).
///
/// If `width` and `height` are 0, bounds are auto-calculated from the cells.
///
/// # Errors
///
/// Returns a formatting error if writing to the formatter fails.
pub fn visualize_cells(
    f: &mut fmt::Formatter<'_>,
    cells: &[(i8, i8)],
    width: usize,
    height: usize,
) -> fmt::Result {
    let (min_col, max_col, min_row, max_row) = if width == 0 || height == 0 {
        // Auto-calculate bounds from cells
        let min_col = cells.iter().map(|(c, _)| *c).min().unwrap_or(0);
        let max_col = cells.iter().map(|(c, _)| *c).max().unwrap_or(0);
        let min_row = cells.iter().map(|(_, r)| *r).min().unwrap_or(0);
        let max_row = cells.iter().map(|(_, r)| *r).max().unwrap_or(0);
        (min_col, max_col, min_row, max_row)
    } else {
        #[allow(clippy::cast_possible_truncation)]
        (0, (width - 1) as i8, 0, (height - 1) as i8)
    };

    for row in (min_row..=max_row).rev() {
        for col in min_col..=max_col {
            if cells.contains(&(col, row)) {
                f.write_char('█')?;
            } else {
                f.write_char('.')?;
            }
        }
        if row > min_row {
            f.write_char('\n')?;
        }
    }
    Ok(())
}
