use crate::game::{Board, FallingPiece, Tetromino};

/// The result of attempting a move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    /// Move was successful, piece is still falling.
    Moved,
    /// Move was blocked (e.g., hit wall or other piece).
    Blocked,
    /// Piece landed and was locked in place.
    Locked { rows_cleared: u32 },
    /// Game is over (piece couldn't spawn).
    GameOver,
}

/// Current phase of the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    /// A piece is falling and can be controlled.
    Falling,
    /// Game has ended.
    GameOver,
}

/// The complete state of a Tetris game.
#[derive(Clone)]
pub struct GameState {
    pub board: Board,
    pub current: Option<FallingPiece>,
    pub next: Tetromino,
    pub rows_cleared: u32,
    pub phase: GamePhase,
}

impl GameState {
    /// Creates a new game with an empty board and random pieces.
    #[must_use]
    pub fn new() -> Self {
        let mut rng = rand::rng();
        Self::new_with_rng(&mut rng)
    }

    /// Creates a new game with an empty board using a provided RNG.
    #[must_use]
    pub fn new_with_rng<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            board: Board::new(),
            current: Some(FallingPiece::spawn(Tetromino::random_with_rng(rng))),
            next: Tetromino::random_with_rng(rng),
            rows_cleared: 0,
            phase: GamePhase::Falling,
        }
    }

    /// Creates a new game with specified starting pieces (useful for testing/AI).
    #[must_use]
    pub const fn with_pieces(current: Tetromino, next: Tetromino) -> Self {
        Self {
            board: Board::new(),
            current: Some(FallingPiece::spawn(current)),
            next,
            rows_cleared: 0,
            phase: GamePhase::Falling,
        }
    }

    /// Creates a game state from an existing board (useful for AI evaluation).
    #[must_use]
    pub fn from_board(board: Board) -> Self {
        let mut rng = rand::rng();
        Self::from_board_with_rng(board, &mut rng)
    }

    /// Creates a game state from an existing board using a provided RNG.
    #[must_use]
    pub fn from_board_with_rng<R: rand::Rng + ?Sized>(board: Board, rng: &mut R) -> Self {
        Self {
            board,
            current: Some(FallingPiece::spawn(Tetromino::random_with_rng(rng))),
            next: Tetromino::random_with_rng(rng),
            rows_cleared: 0,
            phase: GamePhase::Falling,
        }
    }

    /// Returns true if the game is still active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(self.phase, GamePhase::Falling)
    }

    /// Returns true if the game is over.
    #[must_use]
    pub const fn is_game_over(&self) -> bool {
        matches!(self.phase, GamePhase::GameOver)
    }

    /// Attempts to move the current piece left.
    pub fn move_left(&mut self) -> MoveResult {
        self.try_move(-1, 0)
    }

    /// Attempts to move the current piece right.
    pub fn move_right(&mut self) -> MoveResult {
        self.try_move(1, 0)
    }

    /// Attempts to move the current piece down (soft drop).
    pub fn move_down(&mut self) -> MoveResult {
        self.try_move(0, -1)
    }

    /// Attempts to move the piece by the given offset.
    fn try_move(&mut self, dcol: i8, drow: i8) -> MoveResult {
        if self.phase != GamePhase::Falling {
            return MoveResult::GameOver;
        }

        let Some(piece) = self.current else {
            return MoveResult::GameOver;
        };

        let new_piece = piece.moved(dcol, drow);

        if self.board.can_place(&new_piece) {
            self.current = Some(new_piece);
            MoveResult::Moved
        } else if drow < 0 {
            // Moving down and blocked means lock the piece
            self.lock_piece()
        } else {
            MoveResult::Blocked
        }
    }

    /// Attempts to rotate the piece clockwise.
    pub fn rotate_cw(&mut self) -> MoveResult {
        self.try_rotate(true)
    }

    /// Attempts to rotate the piece counter-clockwise.
    pub fn rotate_ccw(&mut self) -> MoveResult {
        self.try_rotate(false)
    }

    /// Attempts rotation with basic wall kicks.
    fn try_rotate(&mut self, clockwise: bool) -> MoveResult {
        if self.phase != GamePhase::Falling {
            return MoveResult::GameOver;
        }

        let Some(piece) = self.current else {
            return MoveResult::GameOver;
        };

        let rotated = if clockwise {
            piece.rotated_cw()
        } else {
            piece.rotated_ccw()
        };

        // Try basic wall kicks: no offset, then left, right
        // This is a simplified kick system; real Tetris uses more complex kicks.
        let kicks = [(0, 0), (-1, 0), (1, 0), (0, 1), (-1, 1), (1, 1)];

        for (dcol, drow) in kicks {
            let kicked = rotated.moved(dcol, drow);
            if self.board.can_place(&kicked) {
                self.current = Some(kicked);
                return MoveResult::Moved;
            }
        }

        MoveResult::Blocked
    }

    /// Hard drops the current piece to the bottom.
    pub fn hard_drop(&mut self) -> MoveResult {
        if self.phase != GamePhase::Falling {
            return MoveResult::GameOver;
        }

        let Some(piece) = self.current else {
            return MoveResult::GameOver;
        };

        if let Some(dropped) = self.board.hard_drop(&piece) {
            self.current = Some(dropped);
            self.lock_piece()
        } else {
            MoveResult::GameOver
        }
    }

    /// Locks the current piece in place and spawns the next piece.
    fn lock_piece(&mut self) -> MoveResult {
        let Some(piece) = self.current.take() else {
            return MoveResult::GameOver;
        };

        // Place the piece on the board
        self.board.place(&piece);

        // Clear any full rows
        let cleared = self.board.clear_full_rows();
        self.rows_cleared += cleared;

        // Spawn the next piece
        let next_piece = FallingPiece::spawn(self.next);
        self.next = Tetromino::random();

        // Check if the new piece can be placed (game over check)
        if self.board.can_place(&next_piece) {
            self.current = Some(next_piece);
            MoveResult::Locked {
                rows_cleared: cleared,
            }
        } else {
            self.phase = GamePhase::GameOver;
            MoveResult::GameOver
        }
    }

    /// Advances the game by one gravity tick (piece falls one row).
    pub fn tick(&mut self) -> MoveResult {
        self.move_down()
    }

    /// Returns the ghost piece position (where piece would land).
    #[must_use]
    pub fn ghost_piece(&self) -> Option<FallingPiece> {
        self.current.and_then(|p| self.board.hard_drop(&p))
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Rotation;

    #[test]
    fn test_new_game() {
        let game = GameState::new();
        assert!(game.is_active());
        assert!(!game.is_game_over());
        assert!(game.current.is_some());
        assert_eq!(game.rows_cleared, 0);
    }

    #[test]
    fn test_move_left_right() {
        let mut game = GameState::with_pieces(Tetromino::T, Tetromino::I);
        let initial_col = game.current.expect("should have piece").col;

        assert_eq!(game.move_right(), MoveResult::Moved);
        assert_eq!(
            game.current.expect("should have piece").col,
            initial_col + 1
        );

        assert_eq!(game.move_left(), MoveResult::Moved);
        assert_eq!(game.current.expect("should have piece").col, initial_col);
    }

    #[test]
    fn test_hard_drop() {
        let mut game = GameState::with_pieces(Tetromino::O, Tetromino::I);
        let result = game.hard_drop();

        assert!(
            matches!(result, MoveResult::Locked { rows_cleared: 0 }),
            "Expected Locked result with 0 rows cleared"
        );

        // Piece should have landed and new piece spawned
        assert!(game.is_active());
        assert!(game.current.is_some());
    }

    #[test]
    fn test_rotation() {
        let mut game = GameState::with_pieces(Tetromino::T, Tetromino::I);
        // Move piece down to give room for rotation
        game.current = Some(FallingPiece {
            tetromino: Tetromino::T,
            rotation: Rotation(0),
            col: 3,
            row: 10, // Middle of board
        });
        let initial_rotation = game.current.expect("should have piece").rotation;

        assert_eq!(game.rotate_cw(), MoveResult::Moved);
        assert_ne!(
            game.current.expect("should have piece").rotation,
            initial_rotation
        );
    }

    #[test]
    fn test_line_clear() {
        let mut game = GameState::with_pieces(Tetromino::I, Tetromino::I);

        // Fill the bottom row except for columns 0-3 (where I piece will go)
        for col in 4..10 {
            game.board[0][col] = true;
        }

        // Move I piece to column 0 and hard drop
        game.current = Some(FallingPiece {
            tetromino: Tetromino::I,
            rotation: Rotation(0),
            col: 0,
            row: 1,
        });

        let result = game.hard_drop();
        assert!(
            matches!(result, MoveResult::Locked { rows_cleared: 1 }),
            "Expected Locked result with 1 row cleared"
        );
    }
}
