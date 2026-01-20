use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct SumOfWells;

impl EvalFn for SumOfWells {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
