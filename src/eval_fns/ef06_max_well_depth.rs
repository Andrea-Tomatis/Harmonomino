use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct MaxWellDepth;

impl EvalFn for MaxWellDepth {
    fn eval(&self, board: &Board) -> u16 {
        (0..Board::WIDTH)
            .map(|col| calculate_well_depth(board, col))
            .max()
            .unwrap_or(0)
    }
}

#[must_use]
pub fn calculate_well_depth(board: &Board, col: usize) -> u16 {
    let mut depth = 0;
    for row in 0..Board::HEIGHT {
        if board[row][col] || board.has_filled_above(row, col) {
            continue;
        }
        // TODO: check if well is allowed to be at edge of the board (I think so)
        let left_filled = if col > 0 { board[row][col - 1] } else { true };
        let right_filled = if col < Board::WIDTH - 1 {
            board[row][col + 1]
        } else {
            true
        };
        if left_filled && right_filled {
            depth += 1;
        }
    }
    depth
}
