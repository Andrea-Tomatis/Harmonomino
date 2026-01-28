use crate::eval_fns::calculate_weighted_score;
use crate::game::{Board, FallingPiece, GameState, Tetromino};
use rayon::prelude::*; // Ensure rayon is imported

const ROWS_CLEARED_WEIGHT: f64 = 100.0;
pub struct Simulator {
    pub weights: [f64; 16],
    pub max_length: usize,
    pub game: GameState,
}

impl Simulator {
    #[must_use]
    pub fn new(weights: [f64; 16], max_length: usize) -> Self {
        Self {
            weights,
            max_length,
            game: GameState::new(),
        }
    }

    /// Simulates a Tetris game using parallelized move evaluation.
    ///
    /// # Panics
    ///
    /// Panics if "Hopefully not".
    ///
    /// # Returns
    ///
    /// Fitness: The total number of rows cleared during the simulation.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn simulate_game(self) -> u32 {
        let mut i: usize = 0;
        let mut game = GameState::new();

        // Use a separate variable to track total lines throughout the simulation
        let mut total_rows_cleared = 0;

        while i < self.max_length {
            let base_piece = FallingPiece::spawn(Tetromino::random());

            // --- PARALLEL SEARCH START ---
            let (best_score, best_state_option, best_rows_cleared) = (0..4u8)
                .into_par_iter()
                .flat_map(|rot_idx| {
                    (0..Board::HEIGHT)
                        .into_par_iter()
                        .map(move |row_idx| (rot_idx, row_idx))
                })
                .map(|(rot_idx, row_idx)| {
                    // --- THREAD LOCAL WORK ---
                    let mut local_max_score = -f64::INFINITY;
                    let mut local_best_state: Option<GameState> = None;
                    // Track the rows cleared SPECIFICALLY for the best move found so far
                    let mut local_best_rows_cleared = 0;

                    let mut rotated_piece = base_piece;
                    rotated_piece.rotation = crate::game::Rotation(rot_idx);
                    rotated_piece.row = row_idx as i8;

                    for col_idx in 0..Board::WIDTH {
                        rotated_piece.col = col_idx as i8;

                        // We calculate current rows for THIS specific column
                        let mut current_rows_cleared = 0;

                        if game.board.can_lock(&rotated_piece) {
                            let mut possible_board = game.board.with_piece(&rotated_piece);

                            current_rows_cleared = possible_board.clear_full_rows();

                            let score = calculate_weighted_score(&possible_board, &self.weights)
                                + f64::from(current_rows_cleared) * ROWS_CLEARED_WEIGHT;

                            if score > local_max_score {
                                local_max_score = score;
                                local_best_state = Some(GameState::from_board(possible_board));
                                // CAPTURE the rows cleared for this high-score move
                                local_best_rows_cleared = current_rows_cleared;
                            }
                        }
                    }
                    // Return the accumulated bests, not the last checked loop variable
                    (local_max_score, local_best_state, local_best_rows_cleared)
                })
                .max_by(|a, b| a.0.partial_cmp(&b.0).expect("Prey this doesn't happen"))
                .expect("Prey this doesn't happen");

            match best_state_option {
                Some(next_state) if best_score > -f64::INFINITY => {
                    game = next_state;
                }
                _ => {
                    break;
                }
            }

            // Accumulate into our local tracker, not the game object (which just got reset)
            total_rows_cleared += best_rows_cleared;

            //Update game state visualization
            game.rows_cleared = total_rows_cleared;

            //let formatted_string = format!("Current State {}:\n{}", i, game.board);
            //println!("{formatted_string}");

            i += 1;
        }

        println!("best result was: {}", total_rows_cleared);
        total_rows_cleared
    }
}
