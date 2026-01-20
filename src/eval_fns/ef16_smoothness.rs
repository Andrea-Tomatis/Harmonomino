use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct Smoothness;

impl EvalFn for Smoothness {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
