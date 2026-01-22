use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct PileHeight;

impl EvalFn for PileHeight {
    #[allow(clippy::cast_possible_truncation)]
    fn eval(&self, board: &Board) -> u8 {
        for (i, row) in board.rows_top_down() {
            if row.iter().any(|&cell| cell) {
                return i as u8;
            }
        }
        20
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
        assert_eq!(EF.eval(&board), 20);
    }

    #[test]
    fn test_pile_height_partial_board() {
        let mut board = Board::new();
        assert_eq!(Board::HEIGHT - 2, 18);
        board[19][0] = true; // Top row
        assert_eq!(EF.eval(&board), 0);
    }
}
