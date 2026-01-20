use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct AltitudeDiff;

impl EvalFn for AltitudeDiff {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
