use rand::rng;
use rand_distr::{Distribution, Normal};

use crate::agent::simulator::Simulator;

pub struct CrossEntropySearch {
    pub n_samples: usize,
    pub n_elite: usize,
    pub max_iter: usize,
    pub means: [f64; 16],
    pub std_devs: [f64; 16],
}

impl CrossEntropySearch {
    pub fn new(n_samples: usize, n_elite: usize, max_iter: usize) -> Self {
        Self {
            n_samples,
            n_elite,
            max_iter,
            means: [0.0; 16],
            std_devs: [10.0; 16],
        }
    }

    pub fn optimize(&mut self, sim_length: usize, n_weights: u8) -> ([f64; 16], f64) {
        let mut rng = rng();

        let mut best_weights = [0.0; 16];
        let mut best_fitness = f64::NEG_INFINITY;

        for _ in 0..self.max_iter {
            let mut candidates = Vec::with_capacity(self.n_samples);

            let normals: Vec<Normal<f64>> = (0..16)
                .map(|i| Normal::new(self.means[i], self.std_devs[i]).unwrap())
                .collect();

            // Sample
            for _ in 0..self.n_samples {
                let mut weights = [0.0; 16];

                for i in 0..16 {
                    weights[i] = normals[i].sample(&mut rng);
                }

                let sim = Simulator::new(weights, sim_length);
                let fitness = f64::from(sim.simulate_game(n_weights));
                candidates.push((weights, fitness));
            }

            // Sort best first
            candidates.sort_by(|a, b| b.1.total_cmp(&a.1));

            // ⭐ track global best
            if candidates[0].1 > best_fitness {
                best_fitness = candidates[0].1;
                best_weights = candidates[0].0;
            }

            let elite = &candidates[..self.n_elite];

            // Update distribution
            for i in 0..16 {
                let mean = elite.iter().map(|(w, _)| w[i]).sum::<f64>() / self.n_elite as f64;

                let var = elite
                    .iter()
                    .map(|(w, _)| (w[i] - mean).powi(2))
                    .sum::<f64>()
                    / self.n_elite as f64;

                self.means[i] = mean;
                self.std_devs[i] = var.sqrt() + 0.01;
            }
        }

        (best_weights, best_fitness)
    }
}
