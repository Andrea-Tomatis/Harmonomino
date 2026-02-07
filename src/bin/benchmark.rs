use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use harmonomino::agent::simulator::Simulator;
use harmonomino::apply_flags;
use harmonomino::cli::Cli;
use harmonomino::harmony::{HarmonySearch, OptimizeConfig, optimize_weights};
use harmonomino::weights;
use rand::SeedableRng;

fn usage() -> String {
    format!(
        "\
Usage: benchmark [OPTIONS]

Runs simulations and prints results.

Options:
  --sim-length <N>      Pieces per simulation game     [default: {}]
  --weights <PATH>      Weights file (repeatable)
  --n-weights <N>       Number of eval functions        [default: {}]
  --averaged            Average fitness over multiple runs
  --averaged-runs <N>   Runs per averaged evaluation   [default: {}]
  --eval                Run deterministic evaluation to CSV
  --seeds <CSV>         Seeds for eval mode (comma-separated)
  --seeds-file <PATH>   Seeds for eval mode (one per line)
  --output-csv <PATH>   Output CSV path for eval mode
  --sweep <PARAM>       Parameter sweep: pitch-adj-rate, iterations, bandwidth, sim-length
  --mass-optimize <N>   Run N optimizations and write results to CSV
  --help                Print this help message

Examples:
  benchmark --weights weights.txt --sim-length 500
  benchmark --sweep iterations --sim-length 100
  benchmark --mass-optimize 100",
        OptimizeConfig::DEFAULT_SIM_LENGTH,
        weights::NUM_WEIGHTS,
        OptimizeConfig::DEFAULT_AVERAGED_RUNS,
    )
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.help_requested() {
        println!("{}", usage());
        return Ok(());
    }

    let mut sim_length: usize = OptimizeConfig::DEFAULT_SIM_LENGTH;
    let mut n_weights: usize = OptimizeConfig::DEFAULT_N_WEIGHTS;
    let mut averaged_runs: usize = OptimizeConfig::DEFAULT_AVERAGED_RUNS;
    apply_flags!(cli, {
        "--sim-length"    => sim_length,
        "--n-weights"     => n_weights,
        "--averaged-runs" => averaged_runs,
    });
    let averaged = cli.has_flag("--averaged");

    if cli.has_flag("--eval") {
        return run_eval(&cli, sim_length, n_weights);
    }

    if let Some(param) = cli.get("--sweep") {
        return sweep_parameter(param, sim_length, n_weights, averaged, averaged_runs);
    }

    if let Some(count_str) = cli.get("--mass-optimize") {
        let count: usize = cli.parse_value("--mass-optimize", count_str)?;
        return mass_optimize(count, sim_length, n_weights, averaged, averaged_runs);
    }

    run_comparison_table(&cli, sim_length, n_weights)
}

/// Default comparison-table mode.
fn run_comparison_table(cli: &Cli, sim_length: usize, n_weights: usize) -> io::Result<()> {
    let weight_paths = cli.get_all("--weights");

    let mut entries: Vec<(String, [f64; weights::NUM_WEIGHTS])> = Vec::new();

    if weight_paths.is_empty() {
        let defaults = ["weights.txt"];
        for name in defaults {
            let path = Path::new(name);
            if path.exists() {
                let w = weights::load(path)?;
                entries.push((name.to_string(), w));
            }
        }
        if entries.is_empty() {
            entries = prompt_and_generate()?;
        }
    } else {
        for path_str in &weight_paths {
            let path = Path::new(path_str);
            let w = weights::load(path)?;
            entries.push(((*path_str).to_string(), w));
        }
    }

    println!("{:<30}| Rows Cleared", "Weights");
    println!("------------------------------+-------------");

    for (label, w) in &entries {
        let sim = Simulator::new(*w, sim_length).with_n_weights(n_weights);
        let rows = sim.simulate_game();
        println!("{label:<30}| {rows}");
    }

    Ok(())
}

/// Deterministic evaluation mode for experiment runs.
fn run_eval(cli: &Cli, sim_length: usize, n_weights: usize) -> io::Result<()> {
    let weight_paths = cli.get_all("--weights");
    if weight_paths.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--weights is required in --eval mode",
        ));
    }

    let output_csv = cli.get("--output-csv").ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "--output-csv is required in --eval mode",
        )
    })?;

    let seeds = if let Some(csv) = cli.get("--seeds") {
        parse_seeds_csv(csv)?
    } else if let Some(path) = cli.get("--seeds-file") {
        parse_seeds_file(Path::new(path))?
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--seeds or --seeds-file is required in --eval mode",
        ));
    };

    let mut writer = BufWriter::new(File::create(output_csv)?);
    writeln!(writer, "weight_id,seed,rows_cleared")?;

    for weight_path in weight_paths {
        let path = Path::new(weight_path);
        let w = weights::load(path)?;
        let weight_id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(weight_path);

        for &seed in &seeds {
            let sim = Simulator::new(w, sim_length).with_n_weights(n_weights);
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let rows = sim.simulate_game_with_rng(&mut rng);
            writeln!(writer, "{weight_id},{seed},{rows}")?;
        }
    }

    Ok(())
}

fn prompt_and_generate() -> io::Result<Vec<(String, [f64; weights::NUM_WEIGHTS])>> {
    eprintln!("No weights files found (tried weights.txt).");
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

    let path = Path::new("weights.txt");
    let result = optimize_weights(&OptimizeConfig::default(), path)?;
    Ok(vec![("weights.txt".to_string(), result.weights)])
}

fn parse_seeds_csv(value: &str) -> io::Result<Vec<u64>> {
    if value.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "seeds CSV must not be empty",
        ));
    }
    value
        .split(',')
        .map(|s| {
            s.trim().parse::<u64>().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("invalid seed '{s}': {e}"),
                )
            })
        })
        .collect()
}

fn parse_seeds_file(path: &Path) -> io::Result<Vec<u64>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut seeds = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let seed: u64 = trimmed.parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid seed '{trimmed}': {e}"),
            )
        })?;
        seeds.push(seed);
    }
    if seeds.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "seeds file did not contain any seeds",
        ));
    }
    Ok(seeds)
}

/// Builds a base config with shared sweep settings.
fn sweep_base_config(
    sim_length: usize,
    n_weights: usize,
    averaged: bool,
    averaged_runs: usize,
) -> OptimizeConfig {
    OptimizeConfig {
        sim_length,
        n_weights,
        averaged,
        averaged_runs,
        ..OptimizeConfig::default()
    }
}

/// Sweeps a single HSA parameter over a range and writes results to CSV.
fn sweep_parameter(
    param: &str,
    sim_length: usize,
    n_weights: usize,
    averaged: bool,
    averaged_runs: usize,
) -> io::Result<()> {
    let base = || sweep_base_config(sim_length, n_weights, averaged, averaged_runs);

    let configs: Vec<(String, OptimizeConfig)> = match param {
        "pitch-adj-rate" => (49..=99)
            .step_by(10)
            .map(|x| {
                let v = f64::from(x) / 100.0;
                (
                    format!("{v}"),
                    OptimizeConfig {
                        pitch_adj_rate: v,
                        ..base()
                    },
                )
            })
            .collect(),
        "iterations" => (100..=1000)
            .step_by(100)
            .map(|v| {
                (
                    format!("{v}"),
                    OptimizeConfig {
                        iterations: v,
                        ..base()
                    },
                )
            })
            .collect(),
        "bandwidth" => [0.05, 0.1, 0.5, 1.0]
            .into_iter()
            .map(|v| {
                (
                    format!("{v}"),
                    OptimizeConfig {
                        bandwidth: v,
                        ..base()
                    },
                )
            })
            .collect(),
        "sim-length" => (100..=2000)
            .step_by(100)
            .map(|v| {
                (
                    format!("{v}"),
                    OptimizeConfig {
                        sim_length: v,
                        ..base()
                    },
                )
            })
            .collect(),
        other => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "unknown sweep parameter '{other}': \
                     expected pitch-adj-rate, iterations, bandwidth, or sim-length"
                ),
            ));
        }
    };

    fs::create_dir_all("results")?;
    let csv_path = format!("results/benchmark_{}.csv", param.replace('-', "_"));
    let mut file = BufWriter::new(File::create(&csv_path)?);

    println!("Sweeping {param} ({} values)...", configs.len());

    for (label, config) in &configs {
        let mut solver = HarmonySearch::new(
            config.memory_size,
            config.iterations,
            config.accept_rate,
            config.pitch_adj_rate,
            config.bandwidth,
        );

        println!("  {param} = {label}");

        let mut rng = rand::rng();
        let result = solver.optimize_with_rng(
            config.sim_length,
            config.bounds,
            config.n_weights,
            config.averaged,
            config.averaged_runs,
            config.early_stop_patience,
            config.early_stop_target,
            &mut rng,
            None,
        );
        writeln!(file, "{label},{:.5}", result.best_score)?;
    }

    println!("Results written to {csv_path}");
    Ok(())
}

/// Runs N independent optimizations and writes all weights + scores to CSV.
fn mass_optimize(
    count: usize,
    sim_length: usize,
    n_weights: usize,
    averaged: bool,
    averaged_runs: usize,
) -> io::Result<()> {
    fs::create_dir_all("results")?;
    let mut file = BufWriter::new(File::create("results/optimized_weights.csv")?);

    writeln!(
        file,
        "Run,Score,{}",
        (1..=weights::NUM_WEIGHTS)
            .map(|i| format!("w{i}"))
            .collect::<Vec<_>>()
            .join(",")
    )?;

    let config = OptimizeConfig {
        sim_length,
        n_weights,
        averaged,
        averaged_runs,
        ..OptimizeConfig::default()
    };

    println!("Running {count} optimizations...");

    for i in 1..=count {
        let mut solver = HarmonySearch::new(
            config.memory_size,
            config.iterations,
            config.accept_rate,
            config.pitch_adj_rate,
            config.bandwidth,
        );

        println!("  Run {i}/{count}");

        let mut rng = rand::rng();
        let result = solver.optimize_with_rng(
            config.sim_length,
            config.bounds,
            config.n_weights,
            config.averaged,
            config.averaged_runs,
            config.early_stop_patience,
            config.early_stop_target,
            &mut rng,
            None,
        );

        writeln!(
            file,
            "{i},{:.5},{}",
            result.best_score,
            result
                .weights
                .iter()
                .map(|w| format!("{w:.5}"))
                .collect::<Vec<_>>()
                .join(",")
        )?;
    }

    println!("Results written to results/optimized_weights.csv");
    Ok(())
}
