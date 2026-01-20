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
    fn eval(&self, board: &Board) -> u8;
}
