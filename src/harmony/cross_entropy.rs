use std::io;
use std::path::Path;

use rand::rng;
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
}

impl CeConfig {
    pub const DEFAULT_N_SAMPLES: usize = 50;
    pub const DEFAULT_N_ELITE: usize = 10;
    pub const DEFAULT_ITERATIONS: usize = 500;
    pub const DEFAULT_SIM_LENGTH: usize = 1000;
    pub const DEFAULT_N_WEIGHTS: usize = 16;
    pub const DEFAULT_AVERAGED_RUNS: usize = 20;
    pub const DEFAULT_INITIAL_STD_DEV: f64 = 10.0;
    pub const DEFAULT_STD_DEV_FLOOR: f64 = 0.01;

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
  --std-dev-floor <F>   Minimum standard deviation      [default: {}]",
            Self::DEFAULT_N_SAMPLES,
            Self::DEFAULT_N_ELITE,
            Self::DEFAULT_ITERATIONS,
            Self::DEFAULT_SIM_LENGTH,
            Self::DEFAULT_N_WEIGHTS,
            Self::DEFAULT_AVERAGED_RUNS,
            Self::DEFAULT_INITIAL_STD_DEV,
            Self::DEFAULT_STD_DEV_FLOOR,
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
        }
    }
}

#[derive(Debug)]
pub struct CrossEntropySearch {
    pub n_samples: usize,
    pub n_elite: usize,
    pub max_iter: usize,
    pub means: [f64; 16],
    pub std_devs: [f64; 16],
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
            means: [0.0; 16],
            std_devs: [initial_std_dev; 16],
        }
    }

    /// Runs the Cross-Entropy Search optimization loop.
    ///
    /// Returns the best weights found and their fitness score.
    ///
    /// # Panics
    ///
    /// Panics if `Normal::new()` fails (only possible with NaN or negative std dev).
    #[allow(clippy::cast_precision_loss)]
    pub fn optimize(
        &mut self,
        sim_length: usize,
        scoring_mode: ScoringMode,
        n_weights: usize,
        averaged: bool,
        averaged_runs: usize,
        std_dev_floor: f64,
    ) -> ([f64; 16], f64) {
        let mut rng = rng();

        let evaluate = |weights: [f64; 16]| -> f64 {
            if averaged {
                let total: f64 = (0..averaged_runs)
                    .map(|_| {
                        let sim = Simulator::new(weights, sim_length, scoring_mode)
                            .with_n_weights(n_weights);
                        f64::from(sim.simulate_game())
                    })
                    .sum();
                total / averaged_runs as f64
            } else {
                let sim =
                    Simulator::new(weights, sim_length, scoring_mode).with_n_weights(n_weights);
                f64::from(sim.simulate_game())
            }
        };

        let mut best_weights = [0.0; 16];
        let mut best_fitness = f64::NEG_INFINITY;

        for iteration in 0..self.max_iter {
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
            let mut candidates: Vec<([f64; 16], f64)> = Vec::with_capacity(self.n_samples);
            for _ in 0..self.n_samples {
                let mut weights = [0.0; 16];
                for (w, normal) in weights.iter_mut().zip(normals.iter()) {
                    *w = normal.sample(&mut rng);
                }
                let fitness = evaluate(weights);
                candidates.push((weights, fitness));
            }

            // Sort by fitness (best first)
            candidates.sort_by(|a, b| b.1.total_cmp(&a.1));

            // Track global best
            if candidates[0].1 > best_fitness {
                best_fitness = candidates[0].1;
                best_weights = candidates[0].0;
            }

            println!("Iteration {iteration}: best={best_fitness:.5}");

            // Update distribution from elite samples
            let elite = &candidates[..self.n_elite];
            let n_elite_f = self.n_elite as f64;

            for i in 0..16 {
                let mean = elite.iter().map(|(w, _)| w[i]).sum::<f64>() / n_elite_f;
                let var = elite
                    .iter()
                    .map(|(w, _)| (w[i] - mean).powi(2))
                    .sum::<f64>()
                    / n_elite_f;

                self.means[i] = mean;
                self.std_devs[i] = var.sqrt().max(std_dev_floor);
            }
        }

        (best_weights, best_fitness)
    }
}

/// Runs Cross-Entropy Search optimization and saves the best weights.
///
/// # Errors
///
/// Returns an error if the weights file cannot be written.
pub fn optimize_weights_ce(config: &CeConfig, output: &Path) -> io::Result<[f64; 16]> {
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

    let (best_weights, best_score) = solver.optimize(
        config.sim_length,
        config.scoring_mode,
        config.n_weights,
        config.averaged,
        config.averaged_runs,
        config.std_dev_floor,
    );

    println!("Best fitness: {best_score:.5}");
    println!(
        "Best weights (first 3): [{:.3}, {:.3}, {:.3}, ...]",
        best_weights[0], best_weights[1], best_weights[2]
    );

    weights::save(output, &best_weights, config.scoring_mode)?;
    println!("Weights saved to {}", output.display());

    Ok(best_weights)
}
