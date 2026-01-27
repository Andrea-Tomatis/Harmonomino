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

        while i < self.max_length {
            let base_piece = FallingPiece::spawn(Tetromino::random());

            // --- PARALLEL SEARCH START ---
            // We combine Rotations (0..4) and Rows (0..Height) into a single parallel iterator.
            // This creates ~80 tasks (4 rotations * 20 rows), fully saturating your CPU.
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

                    let mut rotated_piece = base_piece;
                    rotated_piece.rotation = crate::game::Rotation(rot_idx);
                    rotated_piece.row = row_idx as i8;
                    let mut rows_cleared = 0;

                    // We keep the Column loop serial. It's very tight (only ~10 iterations)
                    // and splitting it further would likely add more overhead than speed.
                    for col_idx in 0..Board::WIDTH {
                        rotated_piece.col = col_idx as i8;
                        rows_cleared = 0;

                        // Check if valid (read-only access to game.board is safe)
                        if game.board.can_lock(&rotated_piece) {
                            let mut possible_board = game.board.with_piece(&rotated_piece);

                            // Game Logic: Clear rows and advance
                            rows_cleared += possible_board.clear_full_rows();
                            let score = calculate_weighted_score(&possible_board, &self.weights)
                                + f64::from(rows_cleared) * ROWS_CLEARED_WEIGHT;

                            if score > local_max_score {
                                local_max_score = score;
                                local_best_state = Some(GameState::from_board(possible_board));
                            }
                        }
                    }
                    (local_max_score, local_best_state, rows_cleared)
                })
                .max_by(|a, b| a.0.partial_cmp(&b.0).expect("Prey this doesn't happen"))
                .expect("Prey this doesn't happen");
            // Reduce: Compare all 80+ results to find the global maximum
            // .reduce(
            //     || (-f64::INFINITY, None, 0),
            //     |a, b| if a.0 > b.0 { a } else { b },
            // );

            // Check if we found a valid move
            // reduce returns (f64, Option<GameState>) directly, not wrapped in another Option.
            match best_state_option {
                Some(next_state) if best_score > -f64::INFINITY => {
                    game = next_state;
                    // Move found, continue loop
                }
                _ => {
                    // No valid move found, or score was -INFINITY
                    break;
                }
            }

            game.rows_cleared += best_rows_cleared;

            // Optional: Visualization
            let formatted_string = format!("Current State {}:\n{}", i, game.board);
            println!("{formatted_string}");

            i += 1;
        }

        game.rows_cleared
    }
}
