use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct ConnectedHoles;

impl EvalFn for ConnectedHoles {
    fn eval(&self, board: &Board) -> u8 {
        let mut holes = 0;
        for (row_idx, row) in board.rows_bottom_up() {
            for (col, &occupied) in row.iter().enumerate() {
                // A hole is an empty cell with at one filled cell directly above it
                if !occupied && row_idx < Board::HEIGHT - 1 && board[row_idx + 1][col] {
                    holes += 1;
                }
            }
        }
        holes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &ConnectedHoles;

    #[test]
    fn test_holes_no_holes() {
        let board = Board::new();
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_holes_with_holes() {
        let mut board = Board::new();
        // Create a hole: empty cell at [0][0] with filled cell above at [1][0]
        board[1][0] = true;
        assert_eq!(EF.eval(&board), 1);
    }

    #[test]
    fn test_holes_multiple_holes() {
        let mut board = Board::new();
        // Create multiple holes
        board[1][0] = true;
        board[5][0] = true;
        assert_eq!(EF.eval(&board), 2);
    }
}
