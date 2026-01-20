use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct SumOfWells;

impl EvalFn for SumOfWells {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
