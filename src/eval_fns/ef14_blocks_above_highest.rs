use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct BlocksAboveHighest;

impl EvalFn for BlocksAboveHighest {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
