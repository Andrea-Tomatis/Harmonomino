//! Board evaluation functions used by the agent.

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
use crate::weights;

pub trait EvalFn {
    /// Evaluates the board and returns a score (0-255).
    fn eval(&self, board: &Board) -> u16;
}

/// Returns a list of all 16 evaluators in the correct order.
/// We use Box<dyn EvalFn> to store different types in one list.
#[must_use]
pub fn get_all_evaluators() -> Vec<Box<dyn EvalFn>> {
    vec![
        Box::new(ef01_pile_height::PileHeight),
        Box::new(ef02_holes::Holes),
        Box::new(ef03_connected_holes::ConnectedHoles),
        Box::new(ef05_altitude_diff::AltitudeDiff),
        Box::new(ef06_max_well_depth::MaxWellDepth),
        Box::new(ef07_sum_of_wells::SumOfWells),
        Box::new(ef09_blocks::Blocks),
        Box::new(ef10_weighted_blocks::WeightedBlocks),
        Box::new(ef11_row_transitions::RowTransitions),
        Box::new(ef12_col_transitions::ColTransitions),
        Box::new(ef13_highest_hole::HighestHole),
        Box::new(ef14_blocks_above_highest::BlocksAboveHighest),
        Box::new(ef15_potential_rows::PotentialRows),
        Box::new(ef16_smoothness::Smoothness),
        Box::new(ef18_row_holes::RowHoles),
        Box::new(ef19_hole_depth::HoleDepth),
    ]
}

/// Calculates the weighted sum of the first `n_weights` heuristics.
#[must_use]
pub fn calculate_weighted_score_n(
    board: &Board,
    weights: &[f64; weights::NUM_WEIGHTS],
    n_weights: usize,
) -> f64 {
    get_all_evaluators()
        .iter()
        .zip(weights.iter())
        .take(n_weights)
        .map(|(evaluator, &weight)| f64::from(evaluator.eval(board)) * weight)
        .sum()
}

/// Calculates the weighted sum of all 16 heuristics.
#[must_use]
pub fn calculate_weighted_score(board: &Board, weights: &[f64; weights::NUM_WEIGHTS]) -> f64 {
    calculate_weighted_score_n(board, weights, weights::NUM_WEIGHTS)
}
