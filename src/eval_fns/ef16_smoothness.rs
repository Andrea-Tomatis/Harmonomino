use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct Smoothness;

impl EvalFn for Smoothness {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
