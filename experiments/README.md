# Experiments

This directory contains the experiment runner and plotting scripts. Python dependencies are managed with `uv`.

## Setup

```bash
uv sync
```

## Run experiments

```bash
uv run python run_experiments.py
```

## Generate plots

```bash
uv run python plot_results.py
```

### Output location

By default plots are written to `experiments/plots/`. You can override this in
`experiments/config.toml`:

```toml
[plots]
output_dir = "../report/figures"
```

## Outputs

- `experiments/results/*.csv`
- `experiments/plots/*.pdf`
- `experiments/weights/*`
