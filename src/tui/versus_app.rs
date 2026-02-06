use std::time::{Duration, Instant};

use ratatui::Frame;
use ratatui::crossterm::event::KeyCode;

use crate::agent::ScoringMode;
use crate::agent::find_best_move;
use crate::game::{Board, GamePhase, GameState, MoveResult, Tetromino};
use crate::weights;

use super::event_loop::TuiApp;
use super::versus_ui;

/// Application state for the versus mode: user vs agent.
pub struct VersusApp {
    pub user_game: GameState,
    pub agent_board: Board,
    pub agent_rows_cleared: u32,
    pub agent_game_over: bool,
    pub weights: [f64; weights::NUM_WEIGHTS],
    pub scoring_mode: ScoringMode,
    pub last_tick: Instant,
    pub tick_rate: Duration,
    pub should_quit: bool,
    pub paused: bool,
}

impl VersusApp {
    /// Creates a new `VersusApp` with the given weights and scoring mode.
    #[must_use]
    pub fn new(weights: [f64; weights::NUM_WEIGHTS], scoring_mode: ScoringMode) -> Self {
        Self {
            user_game: GameState::new(),
            agent_board: Board::new(),
            agent_rows_cleared: 0,
            agent_game_over: false,
            weights,
            scoring_mode,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(500),
            should_quit: false,
            paused: false,
        }
    }

    /// Syncs the agent board to match the user's current state.
    pub const fn sync_agent(&mut self) {
        self.agent_board = self.user_game.board;
        self.agent_rows_cleared = self.user_game.rows_cleared;
        self.agent_game_over = false;
    }

    /// After any user action that may lock a piece, feed the same piece to the agent.
    fn handle_lock(&mut self, result: MoveResult, piece: Option<Tetromino>) {
        if matches!(result, MoveResult::Locked { .. })
            && let Some(tetromino) = piece
        {
            self.agent_place(tetromino);
        }
    }

    /// Lets the agent place the given piece optimally.
    fn agent_place(&mut self, piece: Tetromino) {
        if self.agent_game_over {
            return;
        }
        match find_best_move(
            &self.agent_board,
            piece,
            &self.weights,
            self.scoring_mode,
            weights::NUM_WEIGHTS,
        ) {
            Some((board, rows_cleared)) => {
                self.agent_board = board;
                self.agent_rows_cleared += rows_cleared;
            }
            None => {
                self.agent_game_over = true;
            }
        }
    }
}

impl TuiApp for VersusApp {
    fn game_phase(&self) -> GamePhase {
        self.user_game.phase
    }
    fn last_tick(&self) -> Instant {
        self.last_tick
    }
    fn tick_rate(&self) -> Duration {
        self.tick_rate
    }
    fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn draw(&self, frame: &mut Frame) {
        versus_ui::draw_versus(frame, self);
    }

    fn on_tick(&mut self) {
        if !self.paused && self.user_game.phase == GamePhase::Falling {
            let piece = self.user_game.current.map(|p| p.tetromino);
            let result = self.user_game.tick();
            self.handle_lock(result, piece);
        }
        self.last_tick = Instant::now();
    }

    fn restart(&mut self) {
        self.user_game = GameState::new();
        self.agent_board = Board::new();
        self.agent_rows_cleared = 0;
        self.agent_game_over = false;
        self.last_tick = Instant::now();
        self.paused = false;
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn toggle_pause(&mut self) {
        if self.user_game.is_active() {
            self.paused = !self.paused;
        }
    }

    fn move_left(&mut self) {
        if !self.paused && self.user_game.is_active() {
            self.user_game.move_left();
        }
    }

    fn move_right(&mut self) {
        if !self.paused && self.user_game.is_active() {
            self.user_game.move_right();
        }
    }

    fn soft_drop(&mut self) {
        if !self.paused && self.user_game.is_active() {
            let piece = self.user_game.current.map(|p| p.tetromino);
            let result = self.user_game.move_down();
            self.handle_lock(result, piece);
        }
    }

    fn hard_drop(&mut self) {
        if !self.paused && self.user_game.is_active() {
            let piece = self.user_game.current.map(|p| p.tetromino);
            let result = self.user_game.hard_drop();
            self.handle_lock(result, piece);
        }
    }

    fn rotate_cw(&mut self) {
        if !self.paused && self.user_game.is_active() {
            self.user_game.rotate_cw();
        }
    }

    fn rotate_ccw(&mut self) {
        if !self.paused && self.user_game.is_active() {
            self.user_game.rotate_ccw();
        }
    }

    fn handle_extra_key(&mut self, code: KeyCode) {
        if code == KeyCode::Backspace {
            self.sync_agent();
        }
    }
}
