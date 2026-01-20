use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct ConnectedHoles;

impl EvalFn for ConnectedHoles {
    fn eval(&self, board: &Board) -> u8 {
        todo!()
    }
}
