use std::io;
use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    DefaultTerminal,
};

use harmonomino::game::GamePhase;
use harmonomino::tui::{draw, App};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();
    let poll_timeout = Duration::from_millis(50);

    loop {
        terminal.draw(|frame| draw(frame, &app))?;

        // Poll for input with timeout for responsive controls
        if event::poll(poll_timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            handle_key(&mut app, key.code);
        }

        // Gravity tick
        if app.last_tick.elapsed() >= app.tick_rate {
            app.on_tick();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn handle_key(app: &mut App, code: KeyCode) {
    match code {
        // Quit
        KeyCode::Char('q') | KeyCode::Esc => app.quit(),

        // Restart (only when game over, or anytime with R)
        KeyCode::Char('r') => app.restart(),
        KeyCode::Enter if app.game.phase == GamePhase::GameOver => app.restart(),

        // Pause
        KeyCode::Char('p') => app.toggle_pause(),

        // Movement
        KeyCode::Left | KeyCode::Char('a') => app.move_left(),
        KeyCode::Right | KeyCode::Char('d') => app.move_right(),
        KeyCode::Down | KeyCode::Char('s') => app.soft_drop(),

        // Hard drop
        KeyCode::Char(' ') => app.hard_drop(),

        // Rotation
        KeyCode::Up | KeyCode::Char('x') => app.rotate_cw(),
        KeyCode::Char('z') => app.rotate_ccw(),

        _ => {}
    }
}
