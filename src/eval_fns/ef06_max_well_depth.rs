use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct MaxWellDepth;

impl EvalFn for MaxWellDepth {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
