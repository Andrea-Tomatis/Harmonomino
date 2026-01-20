use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct PotentialRows;

impl EvalFn for PotentialRows {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
