use std::io::{self, Write};
use std::path::Path;
use std::time::Duration;

use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
};

use harmonomino::game::GamePhase;
use harmonomino::harmony::{OptimizeConfig, optimize_weights};
use harmonomino::tui::{VersusApp, draw_versus};
use harmonomino::weights;

const WEIGHTS_PATH: &str = "weights.txt";

fn main() -> io::Result<()> {
    let path = Path::new(WEIGHTS_PATH);

    let weights = if path.exists() {
        weights::load(path)?
    } else {
        prompt_and_generate(path)?
    };

    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal, weights);
    ratatui::restore();
    result
}

fn prompt_and_generate(path: &Path) -> io::Result<[f64; 16]> {
    eprintln!("No weights file found at '{}'.", path.display());
    eprint!("Run optimization to generate one? [y/n] ");
    io::stderr().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if !input.trim().eq_ignore_ascii_case("y") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("'{}' is required to run versus mode", path.display()),
        ));
    }

    optimize_weights(&OptimizeConfig::default(), path)
}

fn run_app(terminal: &mut DefaultTerminal, weights: [f64; 16]) -> io::Result<()> {
    let mut app = VersusApp::new(weights);
    let poll_timeout = Duration::from_millis(50);

    loop {
        terminal.draw(|frame| draw_versus(frame, &app))?;

        if event::poll(poll_timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            handle_key(&mut app, key.code);
        }

        if app.last_tick.elapsed() >= app.tick_rate {
            app.on_tick();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn handle_key(app: &mut VersusApp, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
        KeyCode::Char('r') => app.restart(),
        KeyCode::Enter if app.user_game.phase == GamePhase::GameOver => app.restart(),
        KeyCode::Char('p') => app.toggle_pause(),
        KeyCode::Backspace => app.sync_agent(),
        KeyCode::Left | KeyCode::Char('a') => app.move_left(),
        KeyCode::Right | KeyCode::Char('d') => app.move_right(),
        KeyCode::Down | KeyCode::Char('s') => app.soft_drop(),
        KeyCode::Char(' ') => app.hard_drop(),
        KeyCode::Up | KeyCode::Char('x' | 'w') => app.rotate_cw(),
        KeyCode::Char('z') => app.rotate_ccw(),
        _ => {}
    }
}
