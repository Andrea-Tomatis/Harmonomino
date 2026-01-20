use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct LandingHeight;

impl EvalFn for LandingHeight {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
