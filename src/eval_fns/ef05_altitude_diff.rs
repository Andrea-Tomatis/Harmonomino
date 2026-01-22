use crate::eval_fns::EvalFn;
use crate::game::Board;

/// The difference between the highest occupied cell and the lowest gap
/// directly reachable from the top (i.e., max column height - min column height).
pub struct AltitudeDiff;

impl EvalFn for AltitudeDiff {
    #[allow(clippy::cast_possible_truncation)]
    fn eval(&self, board: &Board) -> u8 {
        let mut max_height = 0usize;
        let mut min_height = Board::HEIGHT;

        for col in 0..Board::WIDTH {
            let height = board.column_height(col);
            max_height = max_height.max(height);
            min_height = min_height.min(height);
        }

        (max_height - min_height) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &AltitudeDiff;

    #[test]
    fn test_altitude_diff_empty_board() {
        let board = Board::new();
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_altitude_diff_flat_surface() {
        let mut board = Board::new();
        // Fill entire bottom row -> all columns have height 1
        for col in 0..Board::WIDTH {
            board[0][col] = true;
        }
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_altitude_diff_single_column() {
        let mut board = Board::new();
        // One column with height 5, rest are 0
        for row in 0..5 {
            board[row][0] = true;
        }
        assert_eq!(EF.eval(&board), 5);
    }

    #[test]
    fn test_altitude_diff_varying_heights() {
        let mut board = Board::new();
        // Col 0: height 3
        board[0][0] = true;
        board[1][0] = true;
        board[2][0] = true;
        // Col 1: height 7
        for row in 0..7 {
            board[row][1] = true;
        }
        // Col 2: height 2
        board[0][2] = true;
        board[1][2] = true;
        // Rest: height 0
        // max=7, min=0 -> diff=7
        assert_eq!(EF.eval(&board), 7);
    }
}
