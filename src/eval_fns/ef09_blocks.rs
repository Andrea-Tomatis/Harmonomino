use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct Eval;

impl EvalFn for Eval {
    #[allow(clippy::cast_possible_truncation)]
    fn eval(&self, board: &Board) -> u8 {
        board.all_cells().filter(|&&cell| cell).count() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &Eval;

    #[test]
    fn test_Eval_empty_board() {
        let board = Board::new();
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_Eval_partial_board() {
        let mut board = Board::new();
        board[0][0] = true;
        board[1][1] = true;
        assert_eq!(EF.eval(&board), 2);
    }
}
