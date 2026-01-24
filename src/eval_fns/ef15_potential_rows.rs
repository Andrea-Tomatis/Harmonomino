use crate::eval_fns::EvalFn;
use crate::game::Board;

/// The number of rows located above the Highest Hole that have more than 8 filled cells (I think).
/// These are rows that are close to being clearable but blocked by a hole below.
pub struct PotentialRows;

impl EvalFn for PotentialRows {
    fn eval(&self, board: &Board) -> u16 {
        let Some(hole_row) = board.highest_hole_row() else {
            return 0;
        };

        let mut count = 0;
        for row in (hole_row + 1)..Board::HEIGHT {
            if board[row].iter().filter(|&&c| c).count() > 8 {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &PotentialRows;

    #[test]
    fn test_no_holes() {
        let board = Board::new();
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_no_potential_rows() {
        let mut board = Board::new();
        // Hole at row 0, sparse row above
        board[1][0] = true;
        board[1][1] = true;
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_one_potential_row() {
        let mut board = Board::new();
        // Create a hole at row 0 (need block above it)
        board[1][0] = true;
        // Fill row 2 with 9 cells (>8)
        for col in 0..9 {
            board[2][col] = true;
        }
        assert_eq!(EF.eval(&board), 1);
    }

    #[test]
    fn test_row_with_exactly_8_not_counted() {
        let mut board = Board::new();
        board[1][0] = true; // Creates hole at row 0
        // Fill row 2 with exactly 8 cells
        for col in 0..8 {
            board[2][col] = true;
        }
        assert_eq!(EF.eval(&board), 0);
    }
}
