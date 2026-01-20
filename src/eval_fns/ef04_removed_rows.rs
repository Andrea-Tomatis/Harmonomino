use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct RemovedRows;

impl EvalFn for RemovedRows {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
