use rand::Rng;

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
    pub fn new(
        hm_mem_size: usize,
        max_iter: usize,
        accept_rate: f64,
        pitch_adj_rate: f64,
        band_width: f64,
    ) -> Self {
        HarmonySearch {
            hm_mem_size,
            max_iter,
            accept_rate,
            pitch_adj_rate,
            band_width,
            harm_mem: Vec::with_capacity(hm_mem_size),
            fitness_mem: Vec::with_capacity(hm_mem_size),
        }
    }

    pub fn optimize<F>(&mut self, objective_func: F, bounds: (f64, f64)) -> ([f64; 16], f64)
    where
        F: Fn(&[f64; 16]) -> f64,
    {
        let mut rng = rand::thread_rng();
        let (min_bound, max_bound) = bounds;

        self.harm_mem.clear();
        self.fitness_mem.clear();

        // Initialization
        for _ in 0..self.hm_mem_size {
            let mut harmony = [0.0; 16];
            for val in harmony.iter_mut() {
                *val = rng.gen_range(min_bound..=max_bound);
            }
            self.harm_mem.push(harmony);
            self.fitness_mem.push(objective_func(&harmony));
        }

        // Optimization Loop
        for _ in 0..self.max_iter {
            let mut new_harmony = [0.0; 16];

            for i in 0..16 {
                // FIXED: Used `r#gen` to escape the keyword
                if rng.r#gen::<f64>() < self.accept_rate {
                    // Memory Consideration
                    let random_mem_idx = rng.gen_range(0..self.hm_mem_size);
                    let mut value = self.harm_mem[random_mem_idx][i];

                    // Pitch Adjustment
                    // FIXED: Used `r#gen` here as well
                    if rng.r#gen::<f64>() < self.pitch_adj_rate {
                        let adjustment = rng.gen_range(-1.0..=1.0) * self.band_width;
                        value += adjustment;
                    }
                    new_harmony[i] = value;
                } else {
                    // Random Selection
                    new_harmony[i] = rng.gen_range(min_bound..=max_bound);
                }
            }

            let new_fitness = objective_func(&new_harmony);

            // Maximization Logic: Find min (worst) to replace
            let (worst_idx, &worst_fitness) = self.fitness_mem
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap();

            if new_fitness > worst_fitness {
                self.harm_mem[worst_idx] = new_harmony;
                self.fitness_mem[worst_idx] = new_fitness;
            }
        }

        // Maximization Logic: Return max (best)
        let (best_idx, &best_fitness) = self.fitness_mem
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();

        (self.harm_mem[best_idx], best_fitness)
    }
}
