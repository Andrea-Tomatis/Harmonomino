use crate::eval_fns::calculate_weighted_score;
use crate::game::{Board, FallingPiece, GameState, Tetromino};
use rayon::prelude::*;

const ROWS_CLEARED_WEIGHT: f64 = 100.0;

pub struct Simulator {
    pub weights: [f64; 16],
    pub max_length: usize,
}

impl Simulator {
    #[must_use]
    pub const fn new(weights: [f64; 16], max_length: usize) -> Self {
        Self {
            weights,
            max_length,
        }
    }

    /// Simulates a Tetris game using parallelized move evaluation.
    ///
    /// Returns the total number of rows cleared during the simulation.
    ///
    /// # Panics
    ///
    /// Panics if score comparison encounters NaN values.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn simulate_game(self) -> u32 {
        let mut i: usize = 0;
        let mut game = GameState::new();
        let mut total_rows_cleared = 0;

        while i < self.max_length {
            let base_piece = FallingPiece::spawn(Tetromino::random());

            let (best_score, best_state_option, best_rows_cleared) = (0..4u8)
                .into_par_iter()
                .flat_map(|rot_idx| {
                    (0..Board::HEIGHT)
                        .into_par_iter()
                        .map(move |row_idx| (rot_idx, row_idx))
                })
                .map(|(rot_idx, row_idx)| {
                    let mut local_max_score = -f64::INFINITY;
                    let mut local_best_state: Option<GameState> = None;
                    let mut local_best_rows_cleared = 0;

                    let mut rotated_piece = base_piece;
                    rotated_piece.rotation = crate::game::Rotation(rot_idx);
                    rotated_piece.row = row_idx as i8;

                    for col_idx in 0..Board::WIDTH {
                        rotated_piece.col = col_idx as i8;

                        if game.board.can_lock(&rotated_piece) {
                            let mut possible_board = game.board.with_piece(&rotated_piece);
                            let current_rows_cleared = possible_board.clear_full_rows();

                            let score = f64::from(current_rows_cleared).mul_add(
                                ROWS_CLEARED_WEIGHT,
                                calculate_weighted_score(&possible_board, &self.weights),
                            );

                            if score > local_max_score {
                                local_max_score = score;
                                local_best_state = Some(GameState::from_board(possible_board));
                                local_best_rows_cleared = current_rows_cleared;
                            }
                        }
                    }
                    (local_max_score, local_best_state, local_best_rows_cleared)
                })
                .max_by(|a, b| a.0.partial_cmp(&b.0).expect("NaN in score comparison"))
                .expect("Empty parallel iterator");

            match best_state_option {
                Some(next_state) if best_score > -f64::INFINITY => {
                    game = next_state;
                }
                _ => {
                    break;
                }
            }

            total_rows_cleared += best_rows_cleared;
            game.rows_cleared = total_rows_cleared;

            i += 1;
        }

        total_rows_cleared
    }
}
