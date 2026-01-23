use crate::eval_fns::EvalFn;
use crate::game::Board;

/// Counts vertically connected gaps as one hole.
/// A connected hole is a run of empty cells in a column that has at least one
/// filled cell above it. Multiple vertically adjacent empty cells count as one.
pub struct Eval;

impl EvalFn for Eval {
    fn eval(&self, board: &Board) -> u8 {
        let mut total = 0;

        for col in 0..Board::WIDTH {
            // Find the highest filled cell in this column
            let mut top_filled = None;
            for row in (0..Board::HEIGHT).rev() {
                if board[row][col] {
                    top_filled = Some(row);
                    break;
                }
            }

            // No filled cells means no holes in this column
            let Some(top) = top_filled else { continue };

            // Count connected hole groups below the top
            let mut in_hole = false;
            for row in (0..top).rev() {
                if board[row][col] {
                    // Filled cell ends the hole group
                    in_hole = false;
                } else {
                    // Empty cell with filled cell above = start of hole group
                    if !in_hole {
                        total += 1;
                        in_hole = true;
                    }
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

    const EF: &dyn EvalFn = &Eval;

    #[test]
    fn test_no_holes_empty_board() {
        let board = Board::new();
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_single_hole() {
        let mut board = Board::new();
        // Block at row 1, empty at row 0 -> 1 connected hole
        board[1][0] = true;
        assert_eq!(EF.eval(&board), 1);
    }

    #[test]
    fn test_vertically_connected_holes_count_as_one() {
        let mut board = Board::new();
        // Block at row 5, empty at rows 0-4 -> still 1 connected hole
        board[5][0] = true;
        assert_eq!(EF.eval(&board), 1);
    }

    #[test]
    fn test_separated_hole_groups() {
        let mut board = Board::new();
        // Column 0: blocks at rows 2 and 5, empty at 0,1 and 3,4
        // This creates 2 connected holes
        board[2][0] = true;
        board[5][0] = true;
        assert_eq!(EF.eval(&board), 2);
    }

    #[test]
    fn test_multiple_columns() {
        let mut board = Board::new();
        // Col 0: block at row 1 -> 1 connected hole
        board[1][0] = true;
        // Col 1: block at row 3 -> 1 connected hole (3 cells)
        board[3][1] = true;
        assert_eq!(EF.eval(&board), 2);
    }
}
