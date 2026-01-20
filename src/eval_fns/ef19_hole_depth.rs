use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct HoleDepth;

impl EvalFn for HoleDepth {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
