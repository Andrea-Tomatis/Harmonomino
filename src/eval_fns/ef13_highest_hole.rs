use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct HighestHole;

impl EvalFn for HighestHole {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
