use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;

use harmonomino::agent::ScoringMode;
use harmonomino::agent::simulator::Simulator;
use harmonomino::apply_flags;
use harmonomino::cli::Cli;
use harmonomino::harmony::{HarmonySearch, OptimizeConfig, optimize_weights};
use harmonomino::weights;

const fn usage() -> &'static str {
    "\
Usage: benchmark [OPTIONS]

Runs a single simulation under each scoring mode and prints a comparison.

Options:
  --sim-length <N>      Pieces per simulation game     [default: 1000]
  --weights <PATH>      Weights file (repeatable)
  --n-weights <N>       Number of eval functions        [default: 16]
  --averaged            Average fitness over multiple runs
  --averaged-runs <N>   Runs per averaged evaluation   [default: 20]
  --sweep <PARAM>       Parameter sweep: pitch-adj-rate, iterations, bandwidth, sim-length
  --mass-optimize <N>   Run N optimizations and write results to CSV
  --help                Print this help message

Examples:
  benchmark --weights weights-full.txt --sim-length 500
  benchmark --sweep iterations --sim-length 100
  benchmark --mass-optimize 100"
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

    if let Some(param) = cli.get("--sweep") {
        return sweep_parameter(param, sim_length, n_weights, averaged, averaged_runs);
    }

    if let Some(count_str) = cli.get("--mass-optimize") {
        let count: usize = cli.parse_value("--mass-optimize", count_str)?;
        return mass_optimize(count, sim_length, n_weights, averaged, averaged_runs);
    }

    run_comparison_table(&cli, sim_length)
}

/// Default comparison-table mode (existing behavior).
fn run_comparison_table(cli: &Cli, sim_length: usize) -> io::Result<()> {
    let weight_paths = cli.get_all("--weights");

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

        let (_, best_score) = solver.optimize(
            config.sim_length,
            config.bounds,
            config.scoring_mode,
            config.n_weights,
            config.averaged,
            config.averaged_runs,
        );
        writeln!(file, "{label},{best_score:.5}")?;
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
        (1..=16)
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

        let (weights, score) = solver.optimize(
            config.sim_length,
            config.bounds,
            config.scoring_mode,
            config.n_weights,
            config.averaged,
            config.averaged_runs,
        );

        writeln!(
            file,
            "{i},{score:.5},{}",
            weights
                .iter()
                .map(|w| format!("{w:.5}"))
                .collect::<Vec<_>>()
                .join(",")
        )?;
    }

    println!("Results written to results/optimized_weights.csv");
    Ok(())
}
