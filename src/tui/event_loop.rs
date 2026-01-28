use std::io;
use std::time::{Duration, Instant};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::game::GamePhase;

/// Shared interface for all TUI app modes (solo, versus, etc.).
pub trait TuiApp {
    fn game_phase(&self) -> GamePhase;
    fn last_tick(&self) -> Instant;
    fn tick_rate(&self) -> Duration;
    fn should_quit(&self) -> bool;

    fn draw(&self, frame: &mut Frame);
    fn on_tick(&mut self);
    fn restart(&mut self);
    fn quit(&mut self);
    fn toggle_pause(&mut self);

    fn move_left(&mut self);
    fn move_right(&mut self);
    fn soft_drop(&mut self);
    fn hard_drop(&mut self);
    fn rotate_cw(&mut self);
    fn rotate_ccw(&mut self);

    /// Handle keys beyond the standard set. Default is a no-op.
    fn handle_extra_key(&mut self, _code: KeyCode) {}
}

/// Runs the shared TUI event loop for any [`TuiApp`].
///
/// # Errors
///
/// Returns an error on terminal I/O failure.
pub fn run_event_loop(terminal: &mut DefaultTerminal, app: &mut impl TuiApp) -> io::Result<()> {
    let poll_timeout = Duration::from_millis(50);

    loop {
        terminal.draw(|frame| app.draw(frame))?;

        if event::poll(poll_timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            handle_key(app, key.code);
        }

        if app.last_tick().elapsed() >= app.tick_rate() {
            app.on_tick();
        }

        if app.should_quit() {
            return Ok(());
        }
    }
}

fn handle_key(app: &mut impl TuiApp, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
        KeyCode::Char('r') => app.restart(),
        KeyCode::Enter if app.game_phase() == GamePhase::GameOver => app.restart(),
        KeyCode::Char('p') => app.toggle_pause(),
        KeyCode::Left | KeyCode::Char('a') => app.move_left(),
        KeyCode::Right | KeyCode::Char('d') => app.move_right(),
        KeyCode::Down | KeyCode::Char('s') => app.soft_drop(),
        KeyCode::Char(' ') => app.hard_drop(),
        KeyCode::Up | KeyCode::Char('x' | 'w') => app.rotate_cw(),
        KeyCode::Char('z') => app.rotate_ccw(),
        other => app.handle_extra_key(other),
    }
}
