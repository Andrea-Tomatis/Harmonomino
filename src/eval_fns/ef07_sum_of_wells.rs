use crate::eval_fns::{EvalFn, ef06_max_well_depth::calculate_well_depth};
use crate::game::Board;

pub struct SumOfWells;

impl EvalFn for SumOfWells {
    fn eval(&self, board: &Board) -> u16 {
        (0..Board::WIDTH)
            .map(|col| calculate_well_depth(board, col))
            .sum()
    }
}
