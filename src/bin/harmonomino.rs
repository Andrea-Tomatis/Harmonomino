use std::io;
use std::path::PathBuf;

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
        "--memory-size"   => config.memory_size,
        "--iterations"    => config.iterations,
        "--accept-rate"   => config.accept_rate,
        "--pitch-adj-rate" => config.pitch_adj_rate,
        "--bandwidth"     => config.bandwidth,
        "--sim-length"    => config.sim_length,
    });

    let output: PathBuf = cli
        .get("--output")
        .map_or_else(|| PathBuf::from("weights.txt"), PathBuf::from);

    optimize_weights(&config, &output)?;
    Ok(())
}
