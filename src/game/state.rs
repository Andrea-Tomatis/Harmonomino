use crate::game::{Board, Tetromino};

pub struct GameState {
    pub board: Board,
    pub current: Tetromino,
    pub next: Tetromino, // NOTE: not to be used yet (probably to expensive)
    pub rows_cleared: u32,
}
