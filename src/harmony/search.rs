use rand::Rng;

use crate::agent::simulator::Simulator;

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
    pub fn optimize(
        &mut self,
        sim_length: usize,
        bounds: (f64, f64),
        averaged: bool,
        n_weights: u8,
    ) -> ([f64; 16], f64) {
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
            self.fitness_mem
                .push(f64::from(sim.simulate_game(n_weights)));
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

            let new_fitness: f64;
            if averaged {
                let mut total_fitness = 0.0;
                for _ in 0..20 {
                    let sim: Simulator = Simulator::new(new_harmony, sim_length);
                    total_fitness += f64::from(sim.simulate_game(n_weights));
                }
                new_fitness = total_fitness / 20.0;
            } else {
                let sim: Simulator = Simulator::new(new_harmony, sim_length);
                new_fitness = f64::from(sim.simulate_game(n_weights));
            };

            //println!("Iteration {cnt}: {new_fitness}");

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
