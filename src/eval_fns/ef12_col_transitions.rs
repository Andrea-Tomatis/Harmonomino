use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct ColTransitions;

impl EvalFn for ColTransitions {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
