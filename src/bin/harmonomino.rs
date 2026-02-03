use harmonomino::harmony::HarmonySearch;

fn main() {
    // Example Objective: Maximize an inverted sphere function (Higher is better)
    // Theoretical Max is 0.0 at [0,0,...]

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
}
