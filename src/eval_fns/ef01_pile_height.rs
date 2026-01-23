use crate::eval_fns::EvalFn;
use crate::game::Board;

/// The row of the topmost block in the board (1-indexed height from bottom).
/// Returns 0 for an empty board.
pub struct PileHeight;

impl EvalFn for PileHeight {
    #[allow(clippy::cast_possible_truncation)]
    fn eval(&self, board: &Board) -> u8 {
        // Find the highest row with any occupied cell
        for row in (0..Board::HEIGHT).rev() {
            if board[row].iter().any(|&cell| cell) {
                return (row + 1) as u8;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &PileHeight;

    #[test]
    fn test_pile_height_empty_board() {
        let board = Board::new();
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_pile_height_bottom_row() {
        let mut board = Board::new();
        board[0][0] = true;
        assert_eq!(EF.eval(&board), 1);
    }

    #[test]
    fn test_pile_height_top_row() {
        let mut board = Board::new();
        board[19][0] = true;
        assert_eq!(EF.eval(&board), 20);
    }

    #[test]
    fn test_pile_height_middle() {
        let mut board = Board::new();
        board[12][5] = true; // Row 12 (0-indexed) -> pile height 13
        assert_eq!(EF.eval(&board), 13);
    }
}
