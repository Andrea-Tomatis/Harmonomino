use std::io;

use harmonomino::tui::{App, run_event_loop};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_event_loop(&mut terminal, &mut App::new());
    ratatui::restore();
    result
}
