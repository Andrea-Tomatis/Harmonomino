mod helpers;

pub mod ef01_pile_height;
pub mod ef02_holes;
pub mod ef03_connected_holes;
pub mod ef05_altitude_diff;
pub mod ef06_max_well_depth;
pub mod ef07_sum_of_wells;
pub mod ef09_blocks;
pub mod ef10_weighted_blocks;
pub mod ef11_row_transitions;
pub mod ef12_col_transitions;
pub mod ef13_highest_hole;
pub mod ef14_blocks_above_highest;
pub mod ef15_potential_rows;
pub mod ef16_smoothness;
pub mod ef18_row_holes;
pub mod ef19_hole_depth;

// Removed: ef04_removed_rows, ef08_landing_height, ef17_eroded_pieces
// (these require game context beyond the board state)

use crate::game::Board;

pub trait EvalFn {
    /// Evaluates the board and returns a score (0-255).
    fn eval(&self, board: &Board) -> u8;
}

/// Returns a list of all 16 evaluators in the correct order.
/// We use Box<dyn EvalFn> to store different types in one list.
pub fn get_all_evaluators() -> Vec<Box<dyn EvalFn>> {
    vec![
        Box::new(ef01_pile_height::Eval),
        Box::new(ef02_holes::Eval),
        Box::new(ef03_connected_holes::Eval),
        Box::new(ef05_altitude_diff::Eval),
        Box::new(ef06_max_well_depth::Eval),
        Box::new(ef07_sum_of_wells::Eval),
        Box::new(ef09_blocks::Eval),
        Box::new(ef10_weighted_blocks::Eval),
        Box::new(ef11_row_transitions::Eval),
        Box::new(ef12_col_transitions::Eval),
        Box::new(ef13_highest_hole::Eval),
        Box::new(ef14_blocks_above_highest::Eval),
        Box::new(ef15_potential_rows::Eval),
        Box::new(ef16_smoothness::Eval),
        Box::new(ef18_row_holes::Eval),
        Box::new(ef19_hole_depth::Eval),
    ]
}

/// Calculates the weighted sum of all heuristics.
pub fn calculate_weighted_score(board: &Board, weights: &[f64; 16]) -> f64 {
    let evaluators = get_all_evaluators();
    let mut total_score = 0.0;

    for (i, evaluator) in evaluators.iter().enumerate() {
        //Run the specific heuristic
        let raw_score = evaluator.eval(board); // returns u8 (0-255)

        // Multiply by the gene weight
        // We cast u8 to f64 for the math
        total_score += (raw_score as f64) * weights[i];
    }

    total_score
}
