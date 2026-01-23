use crate::eval_fns::EvalFn;
use crate::game::Board;

/// The height (1-indexed row) of the topmost hole on the game board.
/// A hole is an empty cell with at least one filled cell above it.
/// Returns 0 if there are no holes.
pub struct Eval;

impl EvalFn for Eval {
    #[allow(clippy::cast_possible_truncation)]
    fn eval(&self, board: &Board) -> u8 {
        board.highest_hole_row().map_or(0, |row| (row + 1) as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &Eval;

    #[test]
    fn test_no_holes() {
        let board = Board::new();
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_single_hole_at_bottom() {
        let mut board = Board::new();
        // Block at row 1, hole at row 0
        board[1][0] = true;
        assert_eq!(EF.eval(&board), 1); // Height 1 (row 0 + 1)
    }

    #[test]
    fn test_hole_higher_up() {
        let mut board = Board::new();
        // Block at row 10, hole at row 9
        board[10][0] = true;
        assert_eq!(EF.eval(&board), 10); // Height 10 (row 9 + 1)
    }

    #[test]
    fn test_multiple_holes_returns_highest() {
        let mut board = Board::new();
        // Block at row 5, holes at 0-4
        board[5][0] = true;
        // Block at row 8 in col 1, holes at 0-7
        board[8][1] = true;
        // Highest hole is at row 7 (height 8)
        assert_eq!(EF.eval(&board), 8);
    }
}
