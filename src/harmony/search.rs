use std::io;
use std::path::Path;

use rand::Rng;

use crate::agent::simulator::Simulator;
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
}

impl OptimizeConfig {
    pub const DEFAULT_MEMORY_SIZE: usize = 5;
    pub const DEFAULT_ITERATIONS: usize = 500;
    pub const DEFAULT_ACCEPT_RATE: f64 = 0.95;
    pub const DEFAULT_PITCH_ADJ_RATE: f64 = 0.99;
    pub const DEFAULT_BANDWIDTH: f64 = 0.1;
    pub const DEFAULT_SIM_LENGTH: usize = 1000;
    pub const DEFAULT_BOUNDS: (f64, f64) = (-1.0, 1.0);

    /// Returns a usage string with the current default values.
    #[must_use]
    pub fn usage() -> String {
        format!(
            "\
Usage: harmonomino [OPTIONS]

Runs Harmony Search optimization to find optimal Tetris agent weights.

Options:
  --memory-size <N>     Harmony memory size           [default: {}]
  --iterations <N>      Number of HSA iterations      [default: {}]
  --accept-rate <F>     Memory consideration rate     [default: {}]
  --pitch-adj-rate <F>  Pitch adjustment rate         [default: {}]
  --bandwidth <F>       Pitch adjustment bandwidth    [default: {}]
  --sim-length <N>      Pieces per simulation game    [default: {}]
  --output <PATH>       Output weights file           [default: weights.txt]
  --help                Print this help message",
            Self::DEFAULT_MEMORY_SIZE,
            Self::DEFAULT_ITERATIONS,
            Self::DEFAULT_ACCEPT_RATE,
            Self::DEFAULT_PITCH_ADJ_RATE,
            Self::DEFAULT_BANDWIDTH,
            Self::DEFAULT_SIM_LENGTH,
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
pub fn optimize_weights(config: &OptimizeConfig, output: &Path) -> io::Result<[f64; 16]> {
    let mut solver = HarmonySearch::new(
        config.memory_size,
        config.iterations,
        config.accept_rate,
        config.pitch_adj_rate,
        config.bandwidth,
    );

    println!(
        "Starting optimization ({} iterations)...",
        config.iterations
    );

    let (best_weights, best_score) = solver.optimize(config.sim_length, config.bounds);

    println!("Best fitness: {best_score:.5}");
    println!(
        "Best weights (first 3): [{:.3}, {:.3}, {:.3}, ...]",
        best_weights[0], best_weights[1], best_weights[2]
    );

    weights::save(output, &best_weights)?;
    println!("Weights saved to {}", output.display());

    Ok(best_weights)
}

#[derive(Debug)]
pub struct HarmonySearch {
    pub hm_mem_size: usize,
    pub max_iter: usize,
    pub accept_rate: f64,
    pub pitch_adj_rate: f64,
    pub band_width: f64,
    pub harm_mem: Vec<[f64; 16]>,
    pub fitness_mem: Vec<f64>,
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

    /// TODO: Short docstring.
    ///
    /// # Panics
    ///
    /// Panics if `fitness_mem` is empty at the end of optimization (happens only when `hm_mem_size` is 0).
    pub fn optimize(&mut self, sim_length: usize, bounds: (f64, f64)) -> ([f64; 16], f64) {
        let mut rng = rand::rng();
        let (min_bound, max_bound) = bounds;

        self.harm_mem.clear();
        self.fitness_mem.clear();

        // Initialization
        for _ in 0..self.hm_mem_size {
            let mut harmony = [0.0; 16];
            for val in &mut harmony {
                *val = rng.random_range(min_bound..=max_bound);
            }
            self.harm_mem.push(harmony);

            let sim = Simulator::new(harmony, sim_length);
            self.fitness_mem.push(f64::from(sim.simulate_game()));
        }

        // Optimization Loop
        for cnt in 0..self.max_iter {
            let mut new_harmony = [0.0; 16];

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

            let sim: Simulator = Simulator::new(new_harmony, sim_length);
            let new_fitness: f64 = f64::from(sim.simulate_game());

            println!("Iteration {cnt}: {new_fitness}");

            // Maximization Logic: Find min (worst) to replace
            let (worst_idx, &worst_fitness) = self
                .fitness_mem
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.total_cmp(b.1)) // NOTE: changed to total_cmp to avaid panic on NaN
                .expect("Fitness memory should not be empty");

            if new_fitness > worst_fitness {
                self.harm_mem[worst_idx] = new_harmony;
                self.fitness_mem[worst_idx] = new_fitness;
            }
        }

        // Maximization Logic: Return max (best)
        let (best_idx, &best_fitness) = self
            .fitness_mem
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1)) // NOTE: changed to total_cmp to avoid panic on NaN
            .expect("Fitness memory should not be empty");

        (self.harm_mem[best_idx], best_fitness)
    }
}
