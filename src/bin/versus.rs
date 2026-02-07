use std::io::{self, Write};
use std::path::Path;

use harmonomino::cli::Cli;
use harmonomino::harmony::{OptimizeConfig, optimize_weights};
use harmonomino::tui::{VersusApp, run_event_loop};
use harmonomino::weights;

const WEIGHTS_PATH: &str = "weights.txt";

fn main() -> io::Result<()> {
    let _cli = Cli::parse();

    let path = Path::new(WEIGHTS_PATH);
    let w = if path.exists() {
        weights::load(path)?
    } else {
        prompt_and_generate(path)?
    };

    let mut terminal = ratatui::init();
    let result = run_event_loop(&mut terminal, &mut VersusApp::new(w));
    ratatui::restore();
    result
}

fn prompt_and_generate(path: &Path) -> io::Result<[f64; weights::NUM_WEIGHTS]> {
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

    optimize_weights(&OptimizeConfig::default(), path).map(|result| result.weights)
}
