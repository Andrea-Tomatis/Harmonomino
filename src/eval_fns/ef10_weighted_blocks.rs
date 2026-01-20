use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct WeightedBlocks;

impl EvalFn for WeightedBlocks {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
