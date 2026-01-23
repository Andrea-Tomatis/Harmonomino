use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct Eval;

impl EvalFn for Eval {
    #[allow(clippy::cast_possible_truncation)]
    fn eval(&self, board: &Board) -> u8 {
        board
            .rows_bottom_up()
            .map(|(i, row)| {
                (row.iter().filter(|&&cell| cell).count() as u8).saturating_mul((i + 1) as u8)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &Eval;

    #[test]
    fn test_blocks_empty_board() {
        let board = Board::new();
        assert_eq!(EF.eval(&board), 0);
    }

    #[test]
    fn test_blocks_partial_board() {
        let mut board = Board::new();
        board[0][0] = true; // Weighs 1
        board[1][1] = true; // Weighs 2
        assert_eq!(EF.eval(&board), 3);
    }
}
