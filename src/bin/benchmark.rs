use harmonomino::agent::simulator::Simulator;
/// Benchark Script for Harmony Search Optimization Algorithm
///
/// This benchmark script evaluates the performance of the Harmony Search optimization algorithm
/// by varying key parameters such as Pitch Adjustment Rate, Max Iterations, and Bandwidth.
/// It also simulates games using optimized weights and runs multiple optimization iterations.
use harmonomino::harmony::HarmonySearch;
use std::fs::File;
use std::io::{BufWriter, Write};

fn benchmark_pitch_adjustment_rate() {
    println!("Benchmarking Pitch Adjustment Rate...\n");

    let mut file = BufWriter::new(
        File::create("benchmark_pitch_adjustment_rate.csv").expect("Unable to create file"),
    );
    for pitch_adj_rate in (49..=99).step_by(10).map(|x| x as f64 / 100.0) {
        let mut solver = HarmonySearch::new(
            5,    // Memory Size
            500,  // Iterations
            0.95, // Accept Rate
            pitch_adj_rate,
            0.1, // Bandwidth
        );

        println!("Benchmarking Pitch Adjustment Rate: {}\n", pitch_adj_rate);

        let (_, best_score) = solver.optimize(500, (-1.0, 1.0), false, 16);
        writeln!(file, "{:.2},{:.5}", pitch_adj_rate, best_score).expect("Unable to write data");
    }
}

fn benchmark_max_iterations() {
    println!("Benchmarking Max Iterations...\n");

    let mut file = BufWriter::new(
        File::create("benchmark_max_iterations.csv").expect("Unable to create file"),
    );
    for max_iter in (100..=1000).step_by(100) {
        let mut solver = HarmonySearch::new(
            5, // Memory Size
            max_iter, 0.95, // Accept Rate
            0.99, // Pitch Adjustment Rate
            0.1,  // Bandwidth
        );

        println!("Benchmarking Max Iterations: {}\n", max_iter);

        let (_, best_score) = solver.optimize(100, (-1.0, 1.0), true, 16);
        writeln!(file, "{},{:.5}", max_iter, best_score).expect("Unable to write data");
    }
}

fn benchmark_bandwidth() {
    println!("Benchmarking Bandwidth...\n");

    let mut file =
        BufWriter::new(File::create("benchmark_bandwidth.csv").expect("Unable to create file"));
    for bandwidth in [0.05, 0.1, 0.5, 1.0] {
        let mut solver = HarmonySearch::new(
            5,    // Memory Size
            500,  // Iterations
            0.95, // Accept Rate
            0.99, // Pitch Adjustment Rate
            bandwidth,
        );

        println!("Benchmarking Bandwidth: {}\n", bandwidth);

        let (_, best_score) = solver.optimize(500, (-1.0, 1.0), false, 16);
        writeln!(file, "{:.2},{:.5}", bandwidth, best_score).expect("Unable to write data");
    }
}

fn simulate_games_with_optimized_weights() {
    println!("Simulating games with optimized weights...\n");

    let mut file =
        BufWriter::new(File::create("simulation_results.csv").expect("Unable to create file"));

    let mut solver = HarmonySearch::new(
        5,    // Memory Size
        1000, // Iterations
        0.95, // Accept Rate
        0.99, // Pitch Adjustment Rate
        0.1,  // Bandwidth
    );

    println!("Running optimization to get weights...\n");
    let (optimized_weights, _) = solver.optimize(100, (-1.0, 1.0), true, 16);

    for num_pieces in (100..=1000).step_by(100) {
        println!("Simulating game with {} pieces...\n", num_pieces);

        let mut total_score = 0.0;
        for _ in 0..30 {
            let sim: Simulator = Simulator::new(optimized_weights.clone(), num_pieces);
            total_score += f64::from(sim.simulate_game(16));
        }
        let score: f64 = total_score / 30.0;

        writeln!(file, "{},{:.5}", num_pieces, score).expect("Unable to write data");
    }
}

fn run_optimization_multiple_times() {
    println!("Running optimization 100 times...\n");

    let mut file =
        BufWriter::new(File::create("optimized_weights.csv").expect("Unable to create file"));

    writeln!(file, "Run,Weights").expect("Unable to write header");

    for i in 1..=100 {
        println!("Running optimization iteration {}...\n", i);

        let mut solver = HarmonySearch::new(
            5,    // Memory Size
            500,  // Iterations
            0.95, // Accept Rate
            0.99, // Pitch Adjustment Rate
            0.1,  // Bandwidth
        );

        let (optimized_weights, score) = solver.optimize(500, (-1.0, 1.0), false, 16);

        writeln!(
            file,
            "{},{},{}",
            i,
            score,
            optimized_weights
                .iter()
                .map(|w| format!("{:.5}", w))
                .collect::<Vec<_>>()
                .join(",")
        )
        .expect("Unable to write data");
    }
}

fn main() {
    //benchmark_pitch_adjustment_rate();
    //benchmark_max_iterations();
    //benchmark_bandwidth();
    simulate_games_with_optimized_weights();
    //run_optimization_multiple_times();
}
