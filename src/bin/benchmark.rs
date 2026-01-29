use std::collections::HashMap;
use std::io::{self, Write};
use std::path::Path;

use harmonomino::agent::ScoringMode;
use harmonomino::agent::simulator::Simulator;
use harmonomino::cli::Cli;
use harmonomino::harmony::{OptimizeConfig, optimize_weights};
use harmonomino::weights;

const fn usage() -> &'static str {
    "\
Usage: benchmark [OPTIONS]

Runs a single simulation under each scoring mode and prints a comparison.

Options:
  --sim-length <N>   Pieces per simulation game  [default: 1000]
  --weights <PATH>   Weights file (repeatable for multiple modes)
  --help             Print this help message

Examples:
  benchmark --weights weights-full.txt --weights weights-heur.txt
  benchmark --weights weights-full.txt --sim-length 500"
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

    let weight_paths = cli.get_all("--weights");

    // Load each weights file and build a map of ScoringMode → weights
    let mut mode_weights: HashMap<ScoringMode, [f64; 16]> = HashMap::new();

    if weight_paths.is_empty() {
        let defaults = ["weights-full.txt", "weights-heur.txt", "weights.txt"];
        for name in defaults {
            let path = Path::new(name);
            if path.exists() {
                let (w, mode) = weights::load(path)?;
                if mode_weights.contains_key(&mode) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("duplicate scoring mode '{mode}' from {name}"),
                    ));
                }
                mode_weights.insert(mode, w);
            }
        }
        if mode_weights.is_empty() {
            prompt_and_generate(&mut mode_weights)?;
        }
    } else {
        for path_str in &weight_paths {
            let path = Path::new(path_str);
            let (w, mode) = weights::load(path)?;
            if mode_weights.contains_key(&mode) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("duplicate scoring mode '{mode}' from {path_str}"),
                ));
            }
            mode_weights.insert(mode, w);
        }
    }

    let modes: &[(ScoringMode, &str)] = &[
        (ScoringMode::Full, "full"),
        (ScoringMode::HeuristicsOnly, "heuristics-only"),
        (ScoringMode::RowsOnly, "rows-only"),
    ];

    println!("{:<19}| Rows Cleared", "Scoring Mode");
    println!("-------------------+-------------");

    for &(mode, label) in modes {
        if mode == ScoringMode::RowsOnly {
            let sim = Simulator::new([0.0; 16], sim_length, mode);
            let rows = sim.simulate_game();
            println!("{label:<19}| {rows}");
        } else if let Some(&w) = mode_weights.get(&mode) {
            let sim = Simulator::new(w, sim_length, mode);
            let rows = sim.simulate_game();
            println!("{label:<19}| {rows}");
        } else {
            println!("{label:<19}| N/A (no matching weights file)");
        }
    }

    Ok(())
}

fn prompt_and_generate(mode_weights: &mut HashMap<ScoringMode, [f64; 16]>) -> io::Result<()> {
    eprintln!("No weights files found (tried weights-full.txt, weights-heur.txt, weights.txt).");
    eprint!("Run optimization to generate weights? [y/n] ");
    io::stderr().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if !input.trim().eq_ignore_ascii_case("y") {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "weights file is required to run benchmark",
        ));
    }

    let modes_to_train: &[(ScoringMode, &str)] = &[
        (ScoringMode::Full, "weights-full.txt"),
        (ScoringMode::HeuristicsOnly, "weights-heur.txt"),
    ];

    for &(mode, output_name) in modes_to_train {
        let path = Path::new(output_name);
        let config = OptimizeConfig {
            scoring_mode: mode,
            ..OptimizeConfig::default()
        };
        let w = optimize_weights(&config, path)?;
        mode_weights.insert(mode, w);
    }

    Ok(())
}
