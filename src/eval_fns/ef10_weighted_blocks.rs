use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct WeightedBlocks;

impl EvalFn for WeightedBlocks {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
