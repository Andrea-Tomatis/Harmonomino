use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct RowHoles;

impl EvalFn for RowHoles {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
