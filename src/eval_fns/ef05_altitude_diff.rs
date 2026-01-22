use crate::eval_fns::{EvalFn, ef01_pile_height::PileHeight};
use crate::game::Board;

const PILE_HEIGHT_FN: &dyn EvalFn = &PileHeight;

pub struct AltitudeDiff;

impl EvalFn for AltitudeDiff {
    #[allow(clippy::cast_possible_truncation)]
    fn eval(&self, board: &Board) -> u8 {
        for row in 0..(Board::HEIGHT - 1) {
            if (0..Board::WIDTH)
                .into_iter()
                .any(|col| !board.has_filled_above(row, col))
            {
                dbg!(PILE_HEIGHT_FN.eval(board));
                dbg!((Board::HEIGHT - row) as u8);
                return (Board::HEIGHT - row) as u8 - PILE_HEIGHT_FN.eval(board);
            }
        }
        // Top row has at least one filled cell (rest of the board is full)
        // Return 0 if the board is full else 1
        u8::from(!board[Board::HEIGHT - 1].iter().any(|&cell| cell))
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
    fn test_altitude_diff_partial_board() {
        let mut board = Board::new();
        board[0][0] = true; // Fill in bottom row
        assert_eq!(EF.eval(&board), 1);

        board[1][0] = true; // Fill second row
        assert_eq!(EF.eval(&board), 2);

        board[19][0] = true; // Fill top row
        assert_eq!(EF.eval(&board), 20);
    }

    #[test]
    fn test_altitude_diff_full_board() {
        let mut board = Board::new();
        for row in 0..Board::HEIGHT {
            for col in 0..Board::WIDTH {
                board[row][col] = true;
            }
        }
        assert_eq!(EF.eval(&board), 0);
    }
}
