use std::fmt;
use std::str::FromStr;

use crate::eval_fns::calculate_weighted_score;
use crate::game::{Board, FallingPiece, GameState, Tetromino};
use rayon::prelude::*;

const ROWS_CLEARED_WEIGHT: f64 = 1.0;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ScoringMode {
    #[default]
    Full,
    HeuristicsOnly,
    RowsOnly,
}

impl FromStr for ScoringMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "full" => Ok(Self::Full),
            "heuristics-only" => Ok(Self::HeuristicsOnly),
            "rows-only" => Ok(Self::RowsOnly),
            other => Err(format!(
                "unknown scoring mode '{other}': expected full, heuristics-only, or rows-only"
            )),
        }
    }
}

impl fmt::Display for ScoringMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full => write!(f, "full"),
            Self::HeuristicsOnly => write!(f, "heuristics-only"),
            Self::RowsOnly => write!(f, "rows-only"),
        }
    }
}

/// Finds the optimal placement for a piece on the given board.
/// Returns the resulting board (with rows cleared) and the number of rows cleared.
///
/// # Panics
///
/// Panics if score comparison encounters NaN values.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn find_best_move(
    board: &Board,
    piece: Tetromino,
    weights: &[f64; 16],
    scoring_mode: ScoringMode,
) -> Option<(Board, u32)> {
    let base_piece = FallingPiece::spawn(piece);

    let all_parallel_placements: Vec<_> = (0..4u8)
        .flat_map(|rot_idx| (0..Board::HEIGHT).map(move |row_idx| (rot_idx, row_idx)))
        .collect();

    let (best_score, best_board, best_rows_cleared) = all_parallel_placements
        .into_par_iter()
        .map(|(rot_idx, row_idx)| {
            let mut local_max_score = -f64::INFINITY;
            let mut local_best_board: Option<Board> = None;
            let mut local_best_rows_cleared = 0;

            let mut rotated_piece = base_piece;
            rotated_piece.rotation = crate::game::Rotation(rot_idx);
            rotated_piece.row = row_idx as i8;

            for col_idx in 0..Board::WIDTH {
                rotated_piece.col = col_idx as i8;

                if board.can_lock(&rotated_piece) {
                    let mut possible_board = board.with_piece(&rotated_piece);
                    let current_rows_cleared = possible_board.clear_full_rows();

                    let score = match scoring_mode {
                        ScoringMode::Full => f64::from(current_rows_cleared).mul_add(
                            ROWS_CLEARED_WEIGHT,
                            calculate_weighted_score(&possible_board, weights),
                        ),
                        ScoringMode::HeuristicsOnly => {
                            calculate_weighted_score(&possible_board, weights)
                        }
                        ScoringMode::RowsOnly => f64::from(current_rows_cleared),
                    };

                    if score > local_max_score {
                        local_max_score = score;
                        local_best_board = Some(possible_board);
                        local_best_rows_cleared = current_rows_cleared;
                    }
                }
            }
            (local_max_score, local_best_board, local_best_rows_cleared)
        })
        .max_by(|a, b| a.0.partial_cmp(&b.0).expect("NaN in score comparison"))
        .expect("Empty parallel iterator");

    if best_score > -f64::INFINITY {
        best_board.map(|b| (b, best_rows_cleared))
    } else {
        None
    }
}

pub struct Simulator {
    pub weights: [f64; 16],
    pub max_length: usize,
    pub scoring_mode: ScoringMode,
}

impl Simulator {
    #[must_use]
    pub const fn new(weights: [f64; 16], max_length: usize, scoring_mode: ScoringMode) -> Self {
        Self {
            weights,
            max_length,
            scoring_mode,
        }
    }

    /// Simulates a Tetris game using parallelized move evaluation.
    ///
    /// Returns the total number of rows cleared during the simulation.
    #[must_use]
    pub fn simulate_game(self) -> u32 {
        let mut game = GameState::new();
        let mut total_rows_cleared = 0;

        for _ in 0..self.max_length {
            let piece = Tetromino::random();

            match find_best_move(&game.board, piece, &self.weights, self.scoring_mode) {
                Some((board, rows_cleared)) => {
                    game = GameState::from_board(board);
                    total_rows_cleared += rows_cleared;
                    game.rows_cleared = total_rows_cleared;
                }
                None => break,
            }
        }

        total_rows_cleared
    }
}
