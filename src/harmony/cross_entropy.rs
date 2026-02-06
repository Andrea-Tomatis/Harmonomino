use std::io::{self, Write};
use std::path::Path;

use rand::Rng;
use rand::SeedableRng;
use rand_distr::{Distribution, Normal};

use crate::agent::simulator::{ScoringMode, Simulator};
use crate::weights;

/// Configuration for a Cross-Entropy Search optimization run.
#[derive(Debug, Clone)]
pub struct CeConfig {
    pub n_samples: usize,
    pub n_elite: usize,
    pub iterations: usize,
    pub sim_length: usize,
    pub scoring_mode: ScoringMode,
    pub n_weights: usize,
    pub averaged: bool,
    pub averaged_runs: usize,
    pub initial_std_dev: f64,
    pub std_dev_floor: f64,
    pub early_stop_patience: usize,
    pub early_stop_target: f64,
}

impl CeConfig {
    pub const DEFAULT_N_SAMPLES: usize = 50;
    pub const DEFAULT_N_ELITE: usize = 10;
    pub const DEFAULT_ITERATIONS: usize = 500;
    pub const DEFAULT_SIM_LENGTH: usize = 1000;
    pub const DEFAULT_N_WEIGHTS: usize = weights::NUM_WEIGHTS;
    pub const DEFAULT_AVERAGED_RUNS: usize = 20;
    pub const DEFAULT_INITIAL_STD_DEV: f64 = 10.0;
    pub const DEFAULT_STD_DEV_FLOOR: f64 = 0.01;
    pub const DEFAULT_EARLY_STOP_TARGET: f64 = f64::INFINITY;

    /// Returns a usage string describing CE-specific options.
    #[must_use]
    pub fn usage() -> String {
        format!(
            "\
Cross-Entropy Search options:
  --n-samples <N>       Candidate samples per iteration [default: {}]
  --n-elite <N>         Elite samples for distribution  [default: {}]
  --iterations <N>      Number of CES iterations        [default: {}]
  --sim-length <N>      Pieces per simulation game      [default: {}]
  --n-weights <N>       Number of eval functions         [default: {}]
  --averaged            Average fitness over multiple runs
  --averaged-runs <N>   Runs per averaged evaluation    [default: {}]
  --initial-std-dev <F> Initial standard deviation      [default: {}]
  --std-dev-floor <F>   Minimum standard deviation      [default: {}]
  --early-stop-patience <N> Stop after N iterations without improvement
  --early-stop-target <F>   Stop once best fitness >= target [default: {}]",
            Self::DEFAULT_N_SAMPLES,
            Self::DEFAULT_N_ELITE,
            Self::DEFAULT_ITERATIONS,
            Self::DEFAULT_SIM_LENGTH,
            Self::DEFAULT_N_WEIGHTS,
            Self::DEFAULT_AVERAGED_RUNS,
            Self::DEFAULT_INITIAL_STD_DEV,
            Self::DEFAULT_STD_DEV_FLOOR,
            Self::DEFAULT_EARLY_STOP_TARGET,
        )
    }
}

impl Default for CeConfig {
    fn default() -> Self {
        Self {
            n_samples: Self::DEFAULT_N_SAMPLES,
            n_elite: Self::DEFAULT_N_ELITE,
            iterations: Self::DEFAULT_ITERATIONS,
            sim_length: Self::DEFAULT_SIM_LENGTH,
            scoring_mode: ScoringMode::default(),
            n_weights: Self::DEFAULT_N_WEIGHTS,
            averaged: false,
            averaged_runs: Self::DEFAULT_AVERAGED_RUNS,
            initial_std_dev: Self::DEFAULT_INITIAL_STD_DEV,
            std_dev_floor: Self::DEFAULT_STD_DEV_FLOOR,
            early_stop_patience: 0,
            early_stop_target: Self::DEFAULT_EARLY_STOP_TARGET,
        }
    }
}

#[derive(Debug)]
pub struct CrossEntropySearch {
    pub n_samples: usize,
    pub n_elite: usize,
    pub max_iter: usize,
    pub means: [f64; weights::NUM_WEIGHTS],
    pub std_devs: [f64; weights::NUM_WEIGHTS],
}

impl CrossEntropySearch {
    /// Creates a new [`CrossEntropySearch`].
    ///
    /// # Panics
    ///
    /// Panics if `n_samples` is zero or `n_elite` exceeds `n_samples`.
    #[must_use]
    pub fn new(n_samples: usize, n_elite: usize, max_iter: usize, initial_std_dev: f64) -> Self {
        assert!(n_samples > 0, "n_samples must be > 0");
        assert!(
            n_elite <= n_samples,
            "n_elite ({n_elite}) must be <= n_samples ({n_samples})"
        );
        Self {
            n_samples,
            n_elite,
            max_iter,
            means: [0.0; weights::NUM_WEIGHTS],
            std_devs: [initial_std_dev; weights::NUM_WEIGHTS],
        }
    }

    /// Runs the Cross-Entropy Search optimization loop.
    ///
    /// Returns the best weights found and their fitness score.
    ///
    /// # Panics
    ///
    /// Panics if `Normal::new()` fails (only possible with NaN or negative std dev).
    pub fn optimize_with_rng<R: Rng + ?Sized>(
        &mut self,
        sim_length: usize,
        scoring_mode: ScoringMode,
        n_weights: usize,
        averaged: bool,
        averaged_runs: usize,
        std_dev_floor: f64,
        early_stop_patience: usize,
        early_stop_target: f64,
        rng: &mut R,
        mut log: Option<&mut dyn Write>,
    ) -> CeOptimizeResult {
        let mut best_weights = [0.0; weights::NUM_WEIGHTS];
        let mut best_fitness = f64::NEG_INFINITY;
        let mut no_improve = 0usize;
        let mut iterations_used = 0usize;

        for iteration in 0..self.max_iter {
            iterations_used = iteration + 1;
            // Build normal distributions from current means and std devs
            let normals: Vec<Normal<f64>> = self
                .means
                .iter()
                .zip(self.std_devs.iter())
                .map(|(&mean, &std_dev)| {
                    Normal::new(mean, std_dev)
                        .expect("Normal distribution parameters must be finite and std_dev >= 0")
                })
                .collect();

            // Sample candidates
            let mut candidates: Vec<([f64; weights::NUM_WEIGHTS], f64)> =
                Vec::with_capacity(self.n_samples);
            for _ in 0..self.n_samples {
                let mut weights = [0.0; weights::NUM_WEIGHTS];
                for (w, normal) in weights.iter_mut().zip(normals.iter()) {
                    *w = normal.sample(rng);
                }
                let fitness = evaluate_weights(
                    rng,
                    weights,
                    sim_length,
                    scoring_mode,
                    n_weights,
                    averaged,
                    averaged_runs,
                );
                candidates.push((weights, fitness));
            }

            // Sort by fitness (best first)
            candidates.sort_by(|a, b| b.1.total_cmp(&a.1));

            // Track global best
            if candidates[0].1 > best_fitness {
                best_fitness = candidates[0].1;
                best_weights = candidates[0].0;
                no_improve = 0;
            } else if early_stop_patience > 0 {
                no_improve += 1;
            }

            println!("Iteration {iteration}: best={best_fitness:.5}");

            // Update distribution from elite samples
            let elite = &candidates[..self.n_elite];
            let n_elite_f = f64::from(u32::try_from(self.n_elite).unwrap_or(u32::MAX));

            for i in 0..weights::NUM_WEIGHTS {
                let mean = elite.iter().map(|(w, _)| w[i]).sum::<f64>() / n_elite_f;
                let var = elite
                    .iter()
                    .map(|(w, _)| (w[i] - mean).powi(2))
                    .sum::<f64>()
                    / n_elite_f;

                self.means[i] = mean;
                self.std_devs[i] = var.sqrt().max(std_dev_floor);
            }

            if let Some(log) = log.as_mut() {
                let (best, mean, worst) = fitness_stats(&candidates);
                let _ = writeln!(log, "{iteration},{best:.5},{mean:.5},{worst:.5}");
            }

            if best_fitness >= early_stop_target {
                break;
            }
            if early_stop_patience > 0 && no_improve >= early_stop_patience {
                break;
            }
        }

        CeOptimizeResult {
            weights: best_weights,
            best_score: best_fitness,
            iterations: iterations_used,
        }
    }
}

/// Runs Cross-Entropy Search optimization and saves the best weights.
///
/// # Errors
///
/// Returns an error if the weights file cannot be written.
pub fn optimize_weights_ce(config: &CeConfig, output: &Path) -> io::Result<CeOptimizeResult> {
    optimize_weights_ce_with_seed(config, output, None, None)
}

/// Runs Cross-Entropy Search optimization with optional seed/logging.
///
/// # Errors
///
/// Returns an error if the weights file or log CSV cannot be written.
pub fn optimize_weights_ce_with_seed(
    config: &CeConfig,
    output: &Path,
    seed: Option<u64>,
    log_csv: Option<&Path>,
) -> io::Result<CeOptimizeResult> {
    seed.map_or_else(
        || {
            let mut rng = rand::rng();
            optimize_weights_ce_with_rng(config, output, &mut rng, log_csv)
        },
        |seed| {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            optimize_weights_ce_with_rng(config, output, &mut rng, log_csv)
        },
    )
}

fn optimize_weights_ce_with_rng<R: Rng + ?Sized>(
    config: &CeConfig,
    output: &Path,
    rng: &mut R,
    log_csv: Option<&Path>,
) -> io::Result<CeOptimizeResult> {
    let mut solver = CrossEntropySearch::new(
        config.n_samples,
        config.n_elite,
        config.iterations,
        config.initial_std_dev,
    );

    println!(
        "Starting CES optimization ({} iterations, n_weights={}, averaged={})...",
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
        config.scoring_mode,
        config.n_weights,
        config.averaged,
        config.averaged_runs,
        config.std_dev_floor,
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

#[derive(Debug, Clone)]
pub struct CeOptimizeResult {
    pub weights: [f64; weights::NUM_WEIGHTS],
    pub best_score: f64,
    pub iterations: usize,
}

fn fitness_stats(candidates: &[([f64; weights::NUM_WEIGHTS], f64)]) -> (f64, f64, f64) {
    if candidates.is_empty() {
        return (f64::NEG_INFINITY, 0.0, f64::INFINITY);
    }
    let best = candidates
        .iter()
        .map(|(_, fitness)| *fitness)
        .max_by(f64::total_cmp)
        .unwrap_or(f64::NEG_INFINITY);
    let worst = candidates
        .iter()
        .map(|(_, fitness)| *fitness)
        .min_by(f64::total_cmp)
        .unwrap_or(f64::INFINITY);
    let denom = f64::from(u32::try_from(candidates.len()).unwrap_or(u32::MAX));
    let mean = candidates.iter().map(|(_, fitness)| *fitness).sum::<f64>() / denom;
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
