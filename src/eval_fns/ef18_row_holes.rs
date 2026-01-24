use crate::eval_fns::EvalFn;
use crate::game::Board;

/// The number of rows that contain at least one hole.
/// A hole is an empty cell with at least one filled cell above it.
pub struct RowHoles;

impl EvalFn for RowHoles {
    fn eval(&self, board: &Board) -> u16 {
        let mut count = 0;

        for row in 0..Board::HEIGHT - 1 {
            for col in 0..Board::WIDTH {
                if !board[row][col] && board.has_filled_above(row, col) {
                    count += 1;
                    break; // Only count each row once
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &RowHoles;

    #[test]
    fn test_no_holes() {
        let board = Board::new();
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_one_row_with_hole() {
        let mut board = Board::new();
        // Block at row 1, hole at row 0
        board[1][0] = true;
        assert_eq!(EF.eval(&board), 1);
    }

    #[test]
    fn test_multiple_holes_same_row() {
        let mut board = Board::new();
        // Blocks at row 1 in columns 0 and 5, holes at row 0
        board[1][0] = true;
        board[1][5] = true;
        // Still only 1 row with holes
        assert_eq!(EF.eval(&board), 1);
    }

    #[test]
    fn test_multiple_rows_with_holes() {
        let mut board = Board::new();
        // Block at row 5 creates holes in rows 0-4
        board[5][0] = true;
        assert_eq!(EF.eval(&board), 5);
    }

    #[test]
    fn test_scattered_holes() {
        let mut board = Board::new();
        // Block at row 2 col 0
        board[2][0] = true;
        // Block at row 4 col 5
        board[4][5] = true;
        // Rows with holes: 0, 1 (from col 0), and 0, 1, 2, 3 (from col 5)
        // Unique rows: 0, 1, 2, 3 = 4 rows
        assert_eq!(EF.eval(&board), 4);
    }
}
