use std::io;
use std::path::PathBuf;

use harmonomino::agent::{ScoringMode, simulator::Simulator};
use harmonomino::apply_flags;
use harmonomino::cli::Cli;
use harmonomino::harmony::{CeConfig, OptimizeConfig, optimize_weights, optimize_weights_ce};

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.help_requested() {
        println!("{}", OptimizeConfig::usage());
        return Ok(());
    }

    let algorithm = cli.get("--algorithm").unwrap_or("hsa");

    match algorithm {
        "hsa" => run_hsa(&cli),
        "ce" => run_ce(&cli),
        other => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unknown algorithm '{other}': expected hsa or ce"),
        )),
    }
}

fn run_hsa(cli: &Cli) -> io::Result<()> {
    let mut config = OptimizeConfig::default();
    apply_flags!(cli, {
        "--memory-size"    => config.memory_size,
        "--iterations"     => config.iterations,
        "--accept-rate"    => config.accept_rate,
        "--pitch-adj-rate" => config.pitch_adj_rate,
        "--bandwidth"      => config.bandwidth,
        "--sim-length"     => config.sim_length,
        "--scoring-mode"   => config.scoring_mode,
        "--n-weights"      => config.n_weights,
        "--averaged-runs"  => config.averaged_runs,
    });
    config.averaged = cli.has_flag("--averaged");

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

fn run_ce(cli: &Cli) -> io::Result<()> {
    let mut config = CeConfig::default();
    apply_flags!(cli, {
        "--n-samples"      => config.n_samples,
        "--n-elite"        => config.n_elite,
        "--iterations"     => config.iterations,
        "--sim-length"     => config.sim_length,
        "--scoring-mode"   => config.scoring_mode,
        "--n-weights"      => config.n_weights,
        "--averaged-runs"  => config.averaged_runs,
        "--initial-std-dev" => config.initial_std_dev,
        "--std-dev-floor"  => config.std_dev_floor,
    });
    config.averaged = cli.has_flag("--averaged");

    let output: PathBuf = cli
        .get("--output")
        .map_or_else(|| PathBuf::from("weights.txt"), PathBuf::from);

    optimize_weights_ce(&config, &output)?;
    Ok(())
}
