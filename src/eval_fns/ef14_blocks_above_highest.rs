use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct BlocksAboveHighest;

impl EvalFn for BlocksAboveHighest {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
