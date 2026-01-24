use crate::eval_fns::calculate_weighted_score;
use crate::game::{Board, FallingPiece, GameState, Tetromino};

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

    #[must_use]
    pub fn simulate_game(self) -> u32 {
        let mut i: usize = 0;
        let mut game = GameState::new();

        while i < self.max_length {
            let mut found_move = false;

            let mut next_state = game.clone();

            let mut max_score = -f64::INFINITY;

            let mut base_piece = FallingPiece::spawn(Tetromino::random());
            base_piece = base_piece.moved(-3, 0); // NOTE: why move piece to the left?

            //try every rotation
            for j in 0..=3 {
                let mut rotated_piece = base_piece;

                for _ in 0..j {
                    rotated_piece = rotated_piece.rotated_cw();
                }

                //try if it fits in any place between the first free line and the last avaible space
                //piece.moved(0,id_top);

                for k in 0..Board::WIDTH {
                    rotated_piece = rotated_piece.moved(0, 1);

                    for _ in 0..Board::HEIGHT {
                        #[allow(clippy::cast_possible_truncation)]
                        let dcol = (-1_i8).pow(k as u32);

                        rotated_piece = rotated_piece.moved(dcol, 0);

                        // NOTE: It is not necessary to first create a piece and then manouver it into position.
                        // Creating a `FallingPiece` directly at the target position would be more efficient and produce cleaner code.

                        if game.board.can_place(&rotated_piece) {
                            let possible_board = game.board.with_piece(&rotated_piece);

                            let score = calculate_weighted_score(&possible_board, &self.weights);
                            if score > max_score {
                                next_state = GameState::from_board(possible_board);
                                max_score = score;
                                found_move = true;
                            }
                        }
                    }
                }
            }

            if !found_move {
                break;
            }

            game = next_state.clone();

            i += 1;
        }

        game.rows_cleared
    }
}
