use std::io;
use std::path::PathBuf;

use harmonomino::apply_flags;
use harmonomino::cli::Cli;
use harmonomino::harmony::{
    CeConfig, OptimizeConfig, optimize_weights_ce_with_seed, optimize_weights_with_seed,
};

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
        "--n-weights"      => config.n_weights,
        "--averaged-runs"  => config.averaged_runs,
        "--early-stop-patience" => config.early_stop_patience,
        "--early-stop-target"   => config.early_stop_target,
    });
    config.averaged = cli.has_flag("--averaged");

    let seed: Option<u64> = cli
        .get("--seed")
        .map(|v| cli.parse_value("--seed", v))
        .transpose()?;
    let log_csv = cli.get("--log-csv").map(PathBuf::from);

    let output: PathBuf = cli
        .get("--output")
        .map_or_else(|| PathBuf::from("weights.txt"), PathBuf::from);

    let _ = optimize_weights_with_seed(&config, &output, seed, log_csv.as_deref())?;
    Ok(())
}

fn run_ce(cli: &Cli) -> io::Result<()> {
    let mut config = CeConfig::default();
    apply_flags!(cli, {
        "--n-samples"      => config.n_samples,
        "--n-elite"        => config.n_elite,
        "--iterations"     => config.iterations,
        "--sim-length"     => config.sim_length,
        "--n-weights"      => config.n_weights,
        "--averaged-runs"  => config.averaged_runs,
        "--initial-std-dev" => config.initial_std_dev,
        "--std-dev-floor"  => config.std_dev_floor,
        "--early-stop-patience" => config.early_stop_patience,
        "--early-stop-target"   => config.early_stop_target,
    });
    config.averaged = cli.has_flag("--averaged");

    let seed: Option<u64> = cli
        .get("--seed")
        .map(|v| cli.parse_value("--seed", v))
        .transpose()?;
    let log_csv = cli.get("--log-csv").map(PathBuf::from);

    let output: PathBuf = cli
        .get("--output")
        .map_or_else(|| PathBuf::from("weights.txt"), PathBuf::from);

    let _ = optimize_weights_ce_with_seed(&config, &output, seed, log_csv.as_deref())?;
    Ok(())
}
