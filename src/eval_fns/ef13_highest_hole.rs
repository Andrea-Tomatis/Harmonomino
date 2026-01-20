use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct HighestHole;

impl EvalFn for HighestHole {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
