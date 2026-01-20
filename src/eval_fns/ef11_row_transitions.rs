use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct RowTransitions;

impl EvalFn for RowTransitions {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
