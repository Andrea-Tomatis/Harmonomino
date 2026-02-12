# Experiments

This directory contains the experiment runner and plotting scripts. Python dependencies are managed with `uv`.

## Recommended (mise)

Run from the repository root:

```bash
mise run experiments
mise run plots
```

## Manual (uv)

Run from `experiments/`:

```bash
uv sync
uv run python run_experiments.py
uv run python plot_results.py
```

### Output location

By default plots are written to `../report/figures` (configured in `experiments/config.toml`).
You can override this path:

```toml
[plots]
output_dir = "../report/figures"
```

## Outputs

- `experiments/results/*.csv`
- `experiments/weights/*`
- `report/figures/*.pdf` (default; location is configurable via `plots.output_dir`)
