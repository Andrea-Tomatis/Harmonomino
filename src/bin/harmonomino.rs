use harmonomino::harmony::CrossEntropySearch;
use harmonomino::harmony::HarmonySearch;

fn main() {
    let mut solver = HarmonySearch::new(
        5,    // Memory Size
        500,  // Iterations
        0.95, // Accept Rate
        0.99, // Pitch Adjust Rate
        0.1,  // Bandwidth
    );

    println!("Starting Optimization (Maximization)...");

    let (best_vars, best_score) = solver.optimize(1000, (-1.0, 1.0), false, 16);

    println!("Best Fitness Found: {best_score:.5}");
    println!(
        "Best Vector (first 3): [{:.3}, {:.3}, {:.3}, ...]",
        best_vars[0], best_vars[1], best_vars[2]
    );

    let mut ce_solver = CrossEntropySearch::new(
        10,  // Population Size
        2,   // Elite Fraction
        100, // Iterations
    );

    println!("Starting Optimization (Cross Entropy)...");

    let (ce_best_vars, ce_best_score) = ce_solver.optimize(1000, 16);

    println!("Best Fitness Found (CE): {ce_best_score:.5}");
    println!(
        "Best Vector (first 3, CE): [{:.3}, {:.3}, {:.3}, ...]",
        ce_best_vars[0], ce_best_vars[1], ce_best_vars[2]
    );
}
