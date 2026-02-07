# Harmonomino

A Tetris AI agent written in Rust that uses metaheuristic optimization to find optimal piece placements. Supports both **Harmony Search Algorithm (HSA)** and **Cross-Entropy Search (CES)** for weight optimization.

The agent evaluates board states using 16 weighted heuristic functions (pile height, holes, wells, row transitions, etc.) and optimizes the weight vector to maximize rows cleared.

Based on work by Romero et al. (Harmony Search for Tetris) and Szita & Lorincz (Noisy Cross-Entropy for Tetris).

## Quick Start

```bash
cargo run                     # Run HSA optimization (default)
cargo run -- --algorithm ce   # Run Cross-Entropy Search
cargo run -- --help           # See all options
```

## Binaries

| Binary | Description |
|--------|-------------|
| `cargo run` | Weight optimization (HSA or CES) |
| `cargo run --bin tetris` | Interactive Tetris game (TUI) |
| `cargo run --bin versus` | Human vs AI side-by-side (TUI) |
| `cargo run --bin benchmark` | Comparison table & parameter sweeps |

## Optimization

```bash
# Harmony Search (default)
cargo run -- --iterations 500 --sim-length 1000

# Cross-Entropy Search
cargo run -- --algorithm ce --iterations 500 --sim-length 1000

# Averaged fitness (reduces noise)
cargo run -- --averaged --averaged-runs 20

# Benchmark parameter sweep
cargo run --bin benchmark -- --sweep iterations --sim-length 100
```

Optimized weights are saved to `weights.txt` and used by the versus mode and benchmark.

## Experiments (uv)

The experiments pipeline lives in `experiments/` and uses `uv` for Python dependencies.

```bash
cd experiments
uv sync
uv run python run_experiments.py
uv run python plot_results.py
```

Outputs:
- `experiments/results/*.csv`
- `experiments/plots/*.pdf`
