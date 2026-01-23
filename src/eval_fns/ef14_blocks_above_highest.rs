use crate::eval_fns::EvalFn;
use crate::game::Board;

/// The number of filled cells above the highest hole.
/// Returns 0 if there are no holes.
pub struct BlocksAboveHighest;

impl EvalFn for BlocksAboveHighest {
    fn eval(&self, board: &Board) -> u8 {
        let Some(hole_row) = board.highest_hole_row() else {
            return 0;
        };

        let mut count = 0;
        for row in (hole_row + 1)..Board::HEIGHT {
            for col in 0..Board::WIDTH {
                if board[row][col] {
                    count += 1;
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

    const EF: &dyn EvalFn = &BlocksAboveHighest;

    #[test]
    fn test_no_holes() {
        let board = Board::new();
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_single_block_above_hole() {
        let mut board = Board::new();
        // Block at row 1, hole at row 0
        board[1][0] = true;
        assert_eq!(EF.eval(&board), 1);
    }

    #[test]
    fn test_multiple_blocks_above_hole() {
        let mut board = Board::new();
        // Hole at row 5, blocks at rows 6, 7, 8
        board[6][0] = true;
        board[7][0] = true;
        board[8][0] = true;
        // Highest hole is at row 5
        assert_eq!(EF.eval(&board), 3);
    }

    #[test]
    fn test_counts_all_columns() {
        let mut board = Board::new();
        // Block at row 2 col 0 creates holes at rows 0,1 in col 0
        board[2][0] = true;
        // Block at row 5 col 1 creates holes at rows 0-4 in col 1
        board[5][1] = true;
        // Highest hole is at row 4 (col 1), only 1 block above it (row 5 col 1)
        assert_eq!(EF.eval(&board), 1);
    }

    #[test]
    fn test_counts_all_blocks_above_highest() {
        let mut board = Board::new();
        // Blocks at rows 5,6,7,8 in col 0 -> holes at rows 0-4
        // Block at row 6 in col 3 -> holes at rows 0-5
        // Block at row 7 in col 5 -> holes at rows 0-6
        board[5][0] = true;
        board[6][0] = true;
        board[7][0] = true;
        board[8][0] = true;
        board[6][3] = true;
        board[7][5] = true;
        // Highest hole is at row 6 (col 5)
        // Blocks above row 6: row 7 col 0, row 7 col 5, row 8 col 0 = 3
        assert_eq!(EF.eval(&board), 3);
    }
}
