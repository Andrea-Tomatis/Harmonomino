use crate::eval_fns::EvalFn;
use crate::game::Board;

/// The sum of all absolute differences of adjacent column heights,
/// plus the difference between the first and last column.
pub struct Eval;

impl EvalFn for Eval {
    fn eval(&self, board: &Board) -> u8 {
        #[allow(clippy::cast_possible_truncation)]
        let heights: [u8; Board::WIDTH] = std::array::from_fn(|col| board.column_height(col) as u8);

        let mut sum = 0;

        // Adjacent column differences
        for i in 0..Board::WIDTH - 1 {
            sum += heights[i].abs_diff(heights[i + 1]);
        }

        // First and last column difference
        // NOTE: Maybe remove dispite paper, I don't see relevance
        sum += heights[0].abs_diff(heights[Board::WIDTH - 1]);

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &Eval;

    #[test]
    fn test_empty_board() {
        let board = Board::new();
        // All heights are 0, all differences are 0
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_flat_surface() {
        let mut board = Board::new();
        for col in 0..Board::WIDTH {
            board[0][col] = true;
        }
        // All heights are 1, all differences are 0
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_single_column() {
        let mut board = Board::new();
        // Column 0 has height 5, rest have 0
        for row in 0..5 {
            board[row][0] = true;
        }
        // |5-0| + |0-0|*8 + |0-5| = 5 + 0 + 5 = 10
        assert_eq!(EF.eval(&board), 10);
    }

    #[test]
    fn test_staircase() {
        let mut board = Board::new();
        // Heights: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
        for col in 0..Board::WIDTH {
            for row in 0..=col {
                board[row][col] = true;
            }
        }
        // Adjacent diffs: all are 1, so 9 * 1 = 9
        // First-last diff: |1-10| = 9
        // Total = 9 + 9 = 18
        assert_eq!(EF.eval(&board), 18);
    }
}
