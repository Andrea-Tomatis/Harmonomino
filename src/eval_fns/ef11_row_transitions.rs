use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct RowTransitions;

impl EvalFn for RowTransitions {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
