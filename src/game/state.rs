use crate::game::{Board, Tetromino};

pub struct GameState {
    pub board: Board,
    pub current: Tetromino,
    pub next: Tetromino,
    pub rows_cleared: u32,
}
