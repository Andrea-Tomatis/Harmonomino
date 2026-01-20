use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct Holes;

impl EvalFn for Holes {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
