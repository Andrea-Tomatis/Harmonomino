use std::io;
use std::path::PathBuf;

use harmonomino::agent::ScoringMode;
use harmonomino::agent::simulator::Simulator;
use harmonomino::cli::Cli;
use harmonomino::weights;

const fn usage() -> &'static str {
    "\
Usage: benchmark [OPTIONS]

Runs a single simulation under each scoring mode and prints a comparison.

Options:
  --sim-length <N>   Pieces per simulation game  [default: 1000]
  --weights <PATH>   Weights file                [default: weights.txt]
  --help             Print this help message"
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.help_requested() {
        println!("{}", usage());
        return Ok(());
    }

    let sim_length: usize = cli
        .get("--sim-length")
        .map(|v| cli.parse_value("--sim-length", v))
        .transpose()?
        .unwrap_or(1000);

    let weights_path: PathBuf = cli
        .get("--weights")
        .map_or_else(|| PathBuf::from("weights.txt"), PathBuf::from);

    let w = weights::load(&weights_path)?;

    let modes: &[(ScoringMode, &str)] = &[
        (ScoringMode::Full, "full"),
        (ScoringMode::HeuristicsOnly, "heuristics-only"),
        (ScoringMode::RowsOnly, "rows-only"),
    ];

    println!("{:<19}| Rows Cleared", "Scoring Mode");
    println!("-------------------+-------------");

    for &(mode, label) in modes {
        let weights_for_mode = if mode == ScoringMode::RowsOnly {
            [0.0; 16]
        } else {
            w
        };
        let sim = Simulator::new(weights_for_mode, sim_length, mode);
        let rows = sim.simulate_game();
        println!("{label:<19}| {rows}");
    }

    Ok(())
}
