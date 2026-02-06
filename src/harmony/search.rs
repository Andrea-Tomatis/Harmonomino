use std::io::{self, Write};
use std::path::Path;

use rand::Rng;
use rand::SeedableRng;

use crate::agent::simulator::{ScoringMode, Simulator};
use crate::weights;

/// Configuration for a full optimization run.
#[derive(Debug, Clone)]
pub struct OptimizeConfig {
    pub memory_size: usize,
    pub iterations: usize,
    pub accept_rate: f64,
    pub pitch_adj_rate: f64,
    pub bandwidth: f64,
    pub sim_length: usize,
    pub bounds: (f64, f64),
    pub scoring_mode: ScoringMode,
    pub n_weights: usize,
    pub averaged: bool,
    pub averaged_runs: usize,
    pub early_stop_patience: usize,
    pub early_stop_target: f64,
}

impl OptimizeConfig {
    pub const DEFAULT_MEMORY_SIZE: usize = 5;
    pub const DEFAULT_ITERATIONS: usize = 500;
    pub const DEFAULT_ACCEPT_RATE: f64 = 0.95;
    pub const DEFAULT_PITCH_ADJ_RATE: f64 = 0.99;
    pub const DEFAULT_BANDWIDTH: f64 = 0.1;
    pub const DEFAULT_SIM_LENGTH: usize = 1000;
    pub const DEFAULT_BOUNDS: (f64, f64) = (-1.0, 1.0);
    pub const DEFAULT_N_WEIGHTS: usize = weights::NUM_WEIGHTS;
    pub const DEFAULT_AVERAGED_RUNS: usize = 20;
    pub const DEFAULT_EARLY_STOP_TARGET: f64 = f64::INFINITY;

    /// Returns a usage string with the current default values.
    #[must_use]
    pub fn usage() -> String {
        format!(
            "\
Usage: harmonomino [OPTIONS]

Runs Harmony Search optimization to find optimal Tetris agent weights.

Options:
  --algorithm <ALG>     Algorithm: hsa, ce            [default: hsa]
  --memory-size <N>     Harmony memory size           [default: {}]
  --iterations <N>      Number of iterations          [default: {}]
  --accept-rate <F>     Memory consideration rate     [default: {}]
  --pitch-adj-rate <F>  Pitch adjustment rate         [default: {}]
  --bandwidth <F>       Pitch adjustment bandwidth    [default: {}]
  --sim-length <N>      Pieces per simulation game    [default: {}]
  --scoring-mode <MODE> Scoring: full, heuristics-only, rows-only [default: full]
  --n-weights <N>       Number of eval functions      [default: {}]
  --averaged            Average fitness over multiple runs
  --averaged-runs <N>   Runs per averaged evaluation  [default: {}]
  --early-stop-patience <N> Stop after N iterations without improvement
  --early-stop-target <F>   Stop once best fitness >= target [default: {}]
  --seed <N>            RNG seed for deterministic runs
  --output <PATH>       Output weights file           [default: weights.txt]
  --log-csv <PATH>      Write per-iteration metrics to CSV
  --help                Print this help message

Cross-Entropy Search options (--algorithm ce):
  --n-samples <N>       Candidate samples per iteration [default: 50]
  --n-elite <N>         Elite samples for distribution  [default: 10]
  --initial-std-dev <F> Initial standard deviation      [default: 10.0]
  --std-dev-floor <F>   Minimum standard deviation      [default: 0.01]",
            Self::DEFAULT_MEMORY_SIZE,
            Self::DEFAULT_ITERATIONS,
            Self::DEFAULT_ACCEPT_RATE,
            Self::DEFAULT_PITCH_ADJ_RATE,
            Self::DEFAULT_BANDWIDTH,
            Self::DEFAULT_SIM_LENGTH,
            Self::DEFAULT_N_WEIGHTS,
            Self::DEFAULT_AVERAGED_RUNS,
            Self::DEFAULT_EARLY_STOP_TARGET,
        )
    }
}

impl Default for OptimizeConfig {
    fn default() -> Self {
        Self {
            memory_size: Self::DEFAULT_MEMORY_SIZE,
            iterations: Self::DEFAULT_ITERATIONS,
            accept_rate: Self::DEFAULT_ACCEPT_RATE,
            pitch_adj_rate: Self::DEFAULT_PITCH_ADJ_RATE,
            bandwidth: Self::DEFAULT_BANDWIDTH,
            sim_length: Self::DEFAULT_SIM_LENGTH,
            bounds: Self::DEFAULT_BOUNDS,
            scoring_mode: ScoringMode::default(),
            n_weights: Self::DEFAULT_N_WEIGHTS,
            averaged: false,
            averaged_runs: Self::DEFAULT_AVERAGED_RUNS,
            early_stop_patience: 0,
            early_stop_target: Self::DEFAULT_EARLY_STOP_TARGET,
        }
    }
}

/// Runs the Harmony Search optimization and saves the best weights to `output`.
///
/// Prints progress to stdout. Returns the best weights found.
///
/// # Errors
///
/// Returns an error if the weights file cannot be written.
pub fn optimize_weights(config: &OptimizeConfig, output: &Path) -> io::Result<OptimizeResult> {
    optimize_weights_with_seed(config, output, None, None)
}

/// Runs the Harmony Search optimization with optional seed/logging.
///
/// # Errors
///
/// Returns an error if the weights file or log CSV cannot be written.
pub fn optimize_weights_with_seed(
    config: &OptimizeConfig,
    output: &Path,
    seed: Option<u64>,
    log_csv: Option<&Path>,
) -> io::Result<OptimizeResult> {
    seed.map_or_else(
        || {
            let mut rng = rand::rng();
            optimize_weights_with_rng(config, output, &mut rng, log_csv)
        },
        |seed| {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            optimize_weights_with_rng(config, output, &mut rng, log_csv)
        },
    )
}

fn optimize_weights_with_rng<R: Rng + ?Sized>(
    config: &OptimizeConfig,
    output: &Path,
    rng: &mut R,
    log_csv: Option<&Path>,
) -> io::Result<OptimizeResult> {
    let mut solver = HarmonySearch::new(
        config.memory_size,
        config.iterations,
        config.accept_rate,
        config.pitch_adj_rate,
        config.bandwidth,
    );

    println!(
        "Starting HSA optimization ({} iterations, n_weights={}, averaged={})...",
        config.iterations, config.n_weights, config.averaged,
    );

    let mut log_writer = if let Some(path) = log_csv {
        let mut file = io::BufWriter::new(std::fs::File::create(path)?);
        writeln!(file, "iteration,best,mean,worst")?;
        Some(file)
    } else {
        None
    };

    let result = solver.optimize_with_rng(
        config.sim_length,
        config.bounds,
        config.scoring_mode,
        config.n_weights,
        config.averaged,
        config.averaged_runs,
        config.early_stop_patience,
        config.early_stop_target,
        rng,
        log_writer.as_mut().map(|writer| writer as &mut dyn Write),
    );

    println!(
        "Best fitness: {:.5} (iterations: {})",
        result.best_score, result.iterations
    );
    println!(
        "Best weights (first 3): [{:.3}, {:.3}, {:.3}, ...]",
        result.weights[0], result.weights[1], result.weights[2]
    );

    weights::save(output, &result.weights, config.scoring_mode)?;
    println!("Weights saved to {}", output.display());

    Ok(result)
}

#[derive(Debug)]
pub struct HarmonySearch {
    pub hm_mem_size: usize,
    pub max_iter: usize,
    pub accept_rate: f64,
    pub pitch_adj_rate: f64,
    pub band_width: f64,
    pub harm_mem: Vec<[f64; weights::NUM_WEIGHTS]>,
    pub fitness_mem: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct OptimizeResult {
    pub weights: [f64; weights::NUM_WEIGHTS],
    pub best_score: f64,
    pub iterations: usize,
}

impl HarmonySearch {
    /// Creates a new [`HarmonySearch`].
    ///
    /// # Panics
    ///
    /// Panics if `hm_mem_size` is zero or if `accept_rate` or `pitch_adj_rate` are not in the range [0, 1].
    #[must_use]
    pub fn new(
        hm_mem_size: usize,
        max_iter: usize,
        accept_rate: f64,
        pitch_adj_rate: f64,
        band_width: f64,
    ) -> Self {
        assert!(hm_mem_size > 0, "Harmony memory size must be > 0");
        assert!(
            (0.0..=1.0).contains(&accept_rate),
            "Accept rate must be in [0, 1]"
        );
        assert!(
            (0.0..=1.0).contains(&pitch_adj_rate),
            "Pitch adjustment rate must be in [0, 1]"
        );
        Self {
            hm_mem_size,
            max_iter,
            accept_rate,
            pitch_adj_rate,
            band_width,
            harm_mem: Vec::with_capacity(hm_mem_size),
            fitness_mem: Vec::with_capacity(hm_mem_size),
        }
    }

    /// Runs the Harmony Search optimization loop.
    ///
    /// # Panics
    ///
    /// Panics if `fitness_mem` is empty at the end of optimization (happens only when `hm_mem_size` is 0).
    pub fn optimize_with_rng<R: Rng + ?Sized>(
        &mut self,
        sim_length: usize,
        bounds: (f64, f64),
        scoring_mode: ScoringMode,
        n_weights: usize,
        averaged: bool,
        averaged_runs: usize,
        early_stop_patience: usize,
        early_stop_target: f64,
        rng: &mut R,
        mut log: Option<&mut dyn Write>,
    ) -> OptimizeResult {
        let (min_bound, max_bound) = bounds;
        let mut best_fitness = f64::NEG_INFINITY;
        let mut no_improve = 0usize;
        let mut iterations_used = 0usize;

        self.harm_mem.clear();
        self.fitness_mem.clear();

        // Initialization
        for _ in 0..self.hm_mem_size {
            let mut harmony = [0.0; weights::NUM_WEIGHTS];
            for val in &mut harmony {
                *val = rng.random_range(min_bound..=max_bound);
            }
            self.harm_mem.push(harmony);
            self.fitness_mem.push(evaluate_weights(
                rng,
                harmony,
                sim_length,
                scoring_mode,
                n_weights,
                averaged,
                averaged_runs,
            ));
        }

        // Optimization Loop
        for cnt in 0..self.max_iter {
            iterations_used = cnt + 1;
            let mut new_harmony = [0.0; weights::NUM_WEIGHTS];

            for (i, note) in new_harmony.iter_mut().enumerate() {
                if rng.random::<f64>() < self.accept_rate {
                    // Memory Consideration
                    let random_mem_idx = rng.random_range(0..self.hm_mem_size);
                    let mut value = self.harm_mem[random_mem_idx][i];

                    // Pitch Adjustment
                    if rng.random::<f64>() < self.pitch_adj_rate {
                        let adjustment = rng.random_range(-1.0..=1.0) * self.band_width; // TODO: maybe Gaussian
                        value += adjustment;
                    }
                    *note = value;
                } else {
                    // Random Selection
                    *note = rng.random_range(min_bound..=max_bound);
                }
            }

            let new_fitness = evaluate_weights(
                rng,
                new_harmony,
                sim_length,
                scoring_mode,
                n_weights,
                averaged,
                averaged_runs,
            );

            println!("Iteration {cnt}: {new_fitness}");

            // Maximization Logic: Find min (worst) to replace
            let (worst_idx, &worst_fitness) = self
                .fitness_mem
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.total_cmp(b.1))
                .expect("Fitness memory should not be empty");

            if new_fitness > worst_fitness {
                self.harm_mem[worst_idx] = new_harmony;
                self.fitness_mem[worst_idx] = new_fitness;
            }

            let (best, mean, worst) = fitness_stats(&self.fitness_mem);
            if let Some(log) = log.as_mut() {
                let _ = writeln!(log, "{cnt},{best:.5},{mean:.5},{worst:.5}");
            }

            if best > best_fitness {
                best_fitness = best;
                no_improve = 0;
            } else if early_stop_patience > 0 {
                no_improve += 1;
            }

            if best_fitness >= early_stop_target {
                break;
            }
            if early_stop_patience > 0 && no_improve >= early_stop_patience {
                break;
            }
        }

        // Maximization Logic: Return max (best)
        let (best_idx, &best_fitness) = self
            .fitness_mem
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .expect("Fitness memory should not be empty");

        OptimizeResult {
            weights: self.harm_mem[best_idx],
            best_score: best_fitness,
            iterations: iterations_used,
        }
    }
}

fn fitness_stats(fitnesses: &[f64]) -> (f64, f64, f64) {
    let best = fitnesses
        .iter()
        .copied()
        .max_by(f64::total_cmp)
        .unwrap_or(f64::NEG_INFINITY);
    let worst = fitnesses
        .iter()
        .copied()
        .min_by(f64::total_cmp)
        .unwrap_or(f64::INFINITY);
    let mean = if fitnesses.is_empty() {
        0.0
    } else {
        fitnesses.iter().sum::<f64>()
            / f64::from(u32::try_from(fitnesses.len()).unwrap_or(u32::MAX))
    };
    (best, mean, worst)
}

fn evaluate_weights<R: Rng + ?Sized>(
    rng: &mut R,
    weights: [f64; weights::NUM_WEIGHTS],
    sim_length: usize,
    scoring_mode: ScoringMode,
    n_weights: usize,
    averaged: bool,
    averaged_runs: usize,
) -> f64 {
    if averaged {
        let total: f64 = (0..averaged_runs)
            .map(|_| {
                let sim =
                    Simulator::new(weights, sim_length, scoring_mode).with_n_weights(n_weights);
                f64::from(sim.simulate_game_with_rng(rng))
            })
            .sum();
        total / f64::from(u32::try_from(averaged_runs).unwrap_or(u32::MAX))
    } else {
        let sim = Simulator::new(weights, sim_length, scoring_mode).with_n_weights(n_weights);
        f64::from(sim.simulate_game_with_rng(rng))
    }
}
