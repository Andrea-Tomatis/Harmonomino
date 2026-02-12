# Harmonomino

A Tetris AI agent written in Rust that uses metaheuristic optimization to find optimal piece placements. Supports both **Harmony Search Algorithm (HSA)** and **Cross-Entropy Search (CES)** for weight optimization.

The agent evaluates board states using 16 weighted heuristic functions (pile height, holes, wells, row transitions, etc.) and optimizes the weight vector to maximize rows cleared.

Based on work by Romero et al. (Harmony Search for Tetris) and Szita & Lorincz (Noisy Cross-Entropy for Tetris).

## Recommended workflow (mise)

Run from the repository root:

```bash
mise run check         # cargo check + clippy + tests
mise run experiments   # run experiment generation
mise run plots         # generate figures and report data exports
mise run report        # compile report/main.pdf
mise run presentation  # compile presentation/main.pdf
mise run pipeline      # full pipeline (experiments -> plots -> report + presentation)
```

## Manual workflow

### Rust binaries (cargo)

Run from the repository root:

```bash
cargo run                           # HSA optimization (default)
cargo run -- --algorithm ce         # Cross-Entropy Search optimization
cargo run -- --help                 # optimizer options
cargo run --bin benchmark -- --sweep iterations --sim-length 100
cargo run --bin tetris              # interactive TUI
cargo run --bin versus              # human vs AI TUI
```

Optimized weights are written to `weights.txt` by default.

### Experiments (uv)

Run from `experiments/`:

```bash
cd experiments
uv sync
uv run python run_experiments.py
uv run python plot_results.py
```

### Report and presentation (typst)

Run from the repository root:

```bash
typst compile report/main.typ
typst compile --root . presentation/main.typ
```

Outputs:
- `experiments/results/*.csv`
- `experiments/weights/*`
- `report/figures/*.pdf`
- `report/data/*`
- `report/main.pdf`
- `presentation/main.pdf`
