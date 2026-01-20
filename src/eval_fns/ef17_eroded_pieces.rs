use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct ErodedPieces;

impl EvalFn for ErodedPieces {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
