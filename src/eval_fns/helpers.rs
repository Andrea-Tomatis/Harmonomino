use crate::game::Board;

impl Board {
    /// Checks if there is at least one filled cell above the given position.
    #[must_use]
    pub fn has_filled_above(&self, row: usize, col: usize) -> bool {
        for r in (row + 1)..Self::HEIGHT {
            if self[r][col] {
                return true;
            }
        }
        false
    }

    /// Returns the row index of the highest hole, or None if no holes exist.
    /// A hole is an empty cell with at least one filled cell above it.
    #[must_use]
    pub fn highest_hole_row(&self) -> Option<usize> {
        for row in (0..Self::HEIGHT - 1).rev() {
            for col in 0..Self::WIDTH {
                if !self[row][col] && self.has_filled_above(row, col) {
                    return Some(row);
                }
            }
        }
        None
    }
}
