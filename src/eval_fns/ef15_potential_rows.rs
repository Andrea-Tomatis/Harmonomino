use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct PotentialRows;

impl EvalFn for PotentialRows {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
