use crate::eval_fns::EvalFn;
use crate::game::Board;

/// The sum of filled cells above each hole.
/// For each hole, count how many filled cells are above it in its column.
pub struct HoleDepth;

impl EvalFn for HoleDepth {
    #[allow(clippy::cast_possible_truncation)]
    fn eval(&self, board: &Board) -> u16 {
        let mut total: u16 = 0;

        for col in 0..Board::WIDTH {
            let mut filled_above: u16 = 0;

            // Scan from top to bottom
            for row in (0..Board::HEIGHT).rev() {
                if board[row][col] {
                    filled_above += 1;
                } else if filled_above > 0 {
                    // This is a hole (empty with filled above)
                    total += filled_above;
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &HoleDepth;

    #[test]
    fn test_no_holes() {
        let board = Board::new();
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_single_hole_depth_1() {
        let mut board = Board::new();
        // Block at row 1, hole at row 0
        board[1][0] = true;
        // Hole at row 0 has 1 filled cell above
        assert_eq!(EF.eval(&board), 1);
    }

    #[test]
    fn test_single_hole_depth_5() {
        let mut board = Board::new();
        // Blocks at rows 1-5, hole at row 0
        for row in 1..6 {
            board[row][0] = true;
        }
        // Hole at row 0 has 5 filled cells above
        assert_eq!(EF.eval(&board), 5);
    }

    #[test]
    fn test_multiple_holes_same_column() {
        let mut board = Board::new();
        // Blocks at rows 1, 3, 5 - holes at 0, 2, 4
        board[1][0] = true;
        board[3][0] = true;
        board[5][0] = true;
        // Hole at row 4: 1 block above (row 5) = 1
        // Hole at row 2: 2 blocks above (rows 3, 5) = 2
        // Hole at row 0: 3 blocks above (rows 1, 3, 5) = 3
        // Total = 1 + 2 + 3 = 6
        assert_eq!(EF.eval(&board), 6);
    }

    #[test]
    fn test_multiple_columns() {
        let mut board = Board::new();
        // Column 0: block at row 1, hole at row 0 -> depth 1
        board[1][0] = true;
        // Column 1: blocks at rows 2,3, hole at row 1 -> depth 2
        board[2][1] = true;
        board[3][1] = true;
        board[0][1] = true; // Not a hole, no block above
        // Total = 1 + 2 = 3
        assert_eq!(EF.eval(&board), 3);
    }
}
