use std::time::{Duration, Instant};

use crate::game::{GamePhase, GameState};

/// Application state wrapping `GameState` with timing for the TUI.
pub struct App {
    pub game: GameState,
    pub last_tick: Instant,
    pub tick_rate: Duration,
    pub should_quit: bool,
    pub paused: bool,
}

impl App {
    /// Creates a new App with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            game: GameState::new(),
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(500),
            should_quit: false,
            paused: false,
        }
    }

    /// Restarts the game.
    pub fn restart(&mut self) {
        self.game = GameState::new();
        self.last_tick = Instant::now();
        self.paused = false;
    }

    /// Handles gravity tick - piece falls one row.
    pub fn on_tick(&mut self) {
        if !self.paused && self.game.phase == GamePhase::Falling {
            self.game.tick();
        }
        self.last_tick = Instant::now();
    }

    /// Moves the current piece left.
    pub fn move_left(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.move_left();
        }
    }

    /// Moves the current piece right.
    pub fn move_right(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.move_right();
        }
    }

    /// Soft drops the current piece (moves down one row).
    pub fn soft_drop(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.move_down();
        }
    }

    /// Hard drops the current piece to the bottom.
    pub fn hard_drop(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.hard_drop();
        }
    }

    /// Rotates the current piece clockwise.
    pub fn rotate_cw(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.rotate_cw();
        }
    }

    /// Rotates the current piece counter-clockwise.
    pub fn rotate_ccw(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.rotate_ccw();
        }
    }

    /// Toggles pause state.
    pub const fn toggle_pause(&mut self) {
        if self.game.is_active() {
            self.paused = !self.paused;
        }
    }

    /// Quits the application.
    pub const fn quit(&mut self) {
        self.should_quit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
