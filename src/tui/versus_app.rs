use std::time::{Duration, Instant};

use crate::agent::find_best_move;
use crate::game::{Board, GamePhase, GameState, MoveResult};

/// Application state for the versus mode: user vs agent.
pub struct VersusApp {
    pub user_game: GameState,
    pub agent_board: Board,
    pub agent_rows_cleared: u32,
    pub agent_game_over: bool,
    pub weights: [f64; 16],
    pub last_tick: Instant,
    pub tick_rate: Duration,
    pub should_quit: bool,
    pub paused: bool,
}

impl VersusApp {
    /// Creates a new `VersusApp` with the given weights.
    #[must_use]
    pub fn new(weights: [f64; 16]) -> Self {
        Self {
            user_game: GameState::new(),
            agent_board: Board::new(),
            agent_rows_cleared: 0,
            agent_game_over: false,
            weights,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(500),
            should_quit: false,
            paused: false,
        }
    }

    /// Restarts both games.
    pub fn restart(&mut self) {
        self.user_game = GameState::new();
        self.agent_board = Board::new();
        self.agent_rows_cleared = 0;
        self.agent_game_over = false;
        self.last_tick = Instant::now();
        self.paused = false;
    }

    /// Syncs the agent board to match the user's current state.
    pub const fn sync_agent(&mut self) {
        self.agent_board = self.user_game.board;
        self.agent_rows_cleared = self.user_game.rows_cleared;
        self.agent_game_over = false;
    }

    /// Handles gravity tick.
    pub fn on_tick(&mut self) {
        if !self.paused && self.user_game.phase == GamePhase::Falling {
            let piece = self.user_game.current.map(|p| p.tetromino);
            let result = self.user_game.tick();
            self.handle_lock(result, piece);
        }
        self.last_tick = Instant::now();
    }

    /// After any user action that may lock a piece, feed the same piece to the agent.
    fn handle_lock(&mut self, result: MoveResult, piece: Option<crate::game::Tetromino>) {
        if matches!(result, MoveResult::Locked { .. })
            && let Some(tetromino) = piece
        {
            self.agent_place(tetromino);
        }
    }

    /// Lets the agent place the given piece optimally.
    fn agent_place(&mut self, piece: crate::game::Tetromino) {
        if self.agent_game_over {
            return;
        }
        match find_best_move(&self.agent_board, piece, &self.weights) {
            Some((board, rows_cleared)) => {
                self.agent_board = board;
                self.agent_rows_cleared += rows_cleared;
            }
            None => {
                self.agent_game_over = true;
            }
        }
    }

    pub fn move_left(&mut self) {
        if !self.paused && self.user_game.is_active() {
            self.user_game.move_left();
        }
    }

    pub fn move_right(&mut self) {
        if !self.paused && self.user_game.is_active() {
            self.user_game.move_right();
        }
    }

    pub fn soft_drop(&mut self) {
        if !self.paused && self.user_game.is_active() {
            let piece = self.user_game.current.map(|p| p.tetromino);
            let result = self.user_game.move_down();
            self.handle_lock(result, piece);
        }
    }

    pub fn hard_drop(&mut self) {
        if !self.paused && self.user_game.is_active() {
            let piece = self.user_game.current.map(|p| p.tetromino);
            let result = self.user_game.hard_drop();
            self.handle_lock(result, piece);
        }
    }

    pub fn rotate_cw(&mut self) {
        if !self.paused && self.user_game.is_active() {
            self.user_game.rotate_cw();
        }
    }

    pub fn rotate_ccw(&mut self) {
        if !self.paused && self.user_game.is_active() {
            self.user_game.rotate_ccw();
        }
    }

    pub const fn toggle_pause(&mut self) {
        if self.user_game.is_active() {
            self.paused = !self.paused;
        }
    }

    pub const fn quit(&mut self) {
        self.should_quit = true;
    }
}
