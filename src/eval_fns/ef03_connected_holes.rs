use crate::eval_fns::EvalFn;
use crate::game::GameState;

pub struct ConnectedHoles;

impl EvalFn for ConnectedHoles {
    fn eval(&self, state: &GameState) -> f64 {
        todo!()
    }
}
