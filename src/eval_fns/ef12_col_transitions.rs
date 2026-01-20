use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct ColTransitions;

impl EvalFn for ColTransitions {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
