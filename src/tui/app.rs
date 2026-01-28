use std::time::{Duration, Instant};

use ratatui::Frame;

use crate::game::{GamePhase, GameState};

use super::event_loop::TuiApp;
use super::ui;

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
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl TuiApp for App {
    fn game_phase(&self) -> GamePhase {
        self.game.phase
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
        ui::draw(frame, self);
    }

    fn on_tick(&mut self) {
        if !self.paused && self.game.phase == GamePhase::Falling {
            self.game.tick();
        }
        self.last_tick = Instant::now();
    }

    fn restart(&mut self) {
        self.game = GameState::new();
        self.last_tick = Instant::now();
        self.paused = false;
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn toggle_pause(&mut self) {
        if self.game.is_active() {
            self.paused = !self.paused;
        }
    }

    fn move_left(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.move_left();
        }
    }

    fn move_right(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.move_right();
        }
    }

    fn soft_drop(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.move_down();
        }
    }

    fn hard_drop(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.hard_drop();
        }
    }

    fn rotate_cw(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.rotate_cw();
        }
    }

    fn rotate_ccw(&mut self) {
        if !self.paused && self.game.is_active() {
            self.game.rotate_ccw();
        }
    }
}
