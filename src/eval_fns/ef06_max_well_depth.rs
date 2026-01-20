use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct MaxWellDepth;

impl EvalFn for MaxWellDepth {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
