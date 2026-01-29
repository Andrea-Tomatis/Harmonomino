use std::io::{self, Write};
use std::path::Path;

use harmonomino::agent::ScoringMode;
use harmonomino::cli::Cli;
use harmonomino::harmony::{OptimizeConfig, optimize_weights};
use harmonomino::tui::{VersusApp, run_event_loop};
use harmonomino::weights;

const WEIGHTS_PATH: &str = "weights.txt";

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let scoring_mode: ScoringMode = cli
        .get("--scoring-mode")
        .map(|v| {
            v.parse()
                .map_err(|e: String| io::Error::new(io::ErrorKind::InvalidInput, e))
        })
        .transpose()?
        .unwrap_or_default();

    let w = if scoring_mode == ScoringMode::RowsOnly {
        [0.0; 16]
    } else {
        let path = Path::new(WEIGHTS_PATH);
        if path.exists() {
            weights::load(path)?
        } else {
            prompt_and_generate(path)?
        }
    };

    let mut terminal = ratatui::init();
    let result = run_event_loop(&mut terminal, &mut VersusApp::new(w, scoring_mode));
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
