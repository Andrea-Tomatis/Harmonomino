use harmonomino::harmony::HarmonySearch;
use harmonomino::agent::simulator::Simulator;


fn main() {
    // Example Objective: Maximize an inverted sphere function (Higher is better)
    // Theoretical Max is 0.0 at [0,0,...]
    let objective = |vars: &[f64; 16]| -> f64 {
        let sum_sq: f64 = vars.iter().map(|x| x * x).sum();
        -sum_sq 
    };

    let mut solver = HarmonySearch::new(
        5,   // Memory Size
        100, // Iterations
        0.95,  // Accept Rate
        0.99,  // Pitch Adjust Rate
        0.01, // Bandwidth
    );

    println!("Starting Optimization (Maximization)...");
    
    let (best_vars, best_score) = solver.optimize(objective, (-10.0, 10.0));

    println!("Best Fitness Found: {:.5}", best_score);
    println!("Best Vector (first 3): [{:.3}, {:.3}, {:.3}, ...]", 
             best_vars[0], best_vars[1], best_vars[2]);
}