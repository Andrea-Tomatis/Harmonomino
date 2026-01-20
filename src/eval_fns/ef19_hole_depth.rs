use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct HoleDepth;

impl EvalFn for HoleDepth {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
