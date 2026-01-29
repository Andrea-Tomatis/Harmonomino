use std::io;
use std::path::PathBuf;

use harmonomino::agent::{ScoringMode, simulator::Simulator};
use harmonomino::apply_flags;
use harmonomino::cli::Cli;
use harmonomino::harmony::{OptimizeConfig, optimize_weights};

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.help_requested() {
        println!("{}", OptimizeConfig::usage());
        return Ok(());
    }

    let mut config = OptimizeConfig::default();
    apply_flags!(cli, {
        "--memory-size"    => config.memory_size,
        "--iterations"     => config.iterations,
        "--accept-rate"    => config.accept_rate,
        "--pitch-adj-rate" => config.pitch_adj_rate,
        "--bandwidth"      => config.bandwidth,
        "--sim-length"     => config.sim_length,
        "--scoring-mode"   => config.scoring_mode,
    });

    let output: PathBuf = cli
        .get("--output")
        .map_or_else(|| PathBuf::from("weights.txt"), PathBuf::from);

    if config.scoring_mode == ScoringMode::RowsOnly {
        println!("Scoring mode: rows-only (skipping HSA optimization)");
        let sim = Simulator::new([0.0; 16], config.sim_length, ScoringMode::RowsOnly);
        let fitness = sim.simulate_game();
        println!("Rows cleared: {fitness}");
        return Ok(());
    }

    optimize_weights(&config, &output)?;
    Ok(())
}
