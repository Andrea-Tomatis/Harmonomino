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

            let mut base_piece: FallingPiece = FallingPiece::spawn(Tetromino::random());

            //try every rotation
            for j in 0..=3 {
                let mut rotated_piece = base_piece;

                //try if it fits in any place between the first free line and the last avaible space

                for k in 0..Board::HEIGHT {
                    for h in 0..Board::WIDTH {
                        rotated_piece.row = k as i8;
                        rotated_piece.col = h as i8;
                        rotated_piece.rotation = crate::game::Rotation(j as u8);

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

            // Visualization and loop logic
            let formatted_string = format!("Current State {}:\n{}", i, game.board);
            println!("{}", formatted_string);

            i += 1;
        }

        game.rows_cleared
    }
}
