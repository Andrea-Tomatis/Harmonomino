import csv
import random
import shutil
import subprocess
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BASE_DIR = Path(__file__).resolve().parent
CONFIG_PATH = BASE_DIR / "config.toml"
RESULTS_DIR = BASE_DIR / "results"
PLOTS_DIR = BASE_DIR / "plots"
WEIGHTS_DIR = BASE_DIR / "weights"


def load_config() -> dict:
    with CONFIG_PATH.open("rb") as f:
        return tomllib.load(f)


def ensure_dirs() -> None:
    for path in [RESULTS_DIR, PLOTS_DIR, WEIGHTS_DIR / "hsa", WEIGHTS_DIR / "ces", WEIGHTS_DIR / "baselines"]:
        path.mkdir(parents=True, exist_ok=True)


def run_cmd(cmd: list[str]) -> None:
    print("+", " ".join(cmd))
    subprocess.run(cmd, cwd=ROOT, check=True)


def write_weights(path: Path, weights: list[float]) -> None:
    lines = [f"{w}\n" for w in weights]
    path.write_text("".join(lines))


# ---------------------------------------------------------------------------
# Training
# ---------------------------------------------------------------------------


def train_hsa(seed: int, cfg: dict) -> Path:
    output = WEIGHTS_DIR / "hsa" / f"seed-{seed}.txt"
    log_csv = RESULTS_DIR / f"convergence_hsa_seed-{seed}.csv"
    if output.exists() and log_csv.exists():
        print(f"Skipping HSA seed {seed} (outputs exist).")
        return output
    cmd = [
        "cargo", "run", "--release", "--bin", "harmonomino", "--",
        "--algorithm", "hsa",
        "--seed", str(seed),
        "--iterations", str(cfg["iterations"]),
        "--memory-size", str(cfg["memory_size"]),
        "--accept-rate", str(cfg["accept_rate"]),
        "--pitch-adj-rate", str(cfg["pitch_adj_rate"]),
        "--bandwidth", str(cfg["bandwidth"]),
        "--sim-length", str(cfg["sim_length"]),
        "--n-weights", str(cfg["n_weights"]),
        "--averaged-runs", str(cfg["averaged_runs"]),
        "--output", str(output),
        "--log-csv", str(log_csv),
    ]
    if cfg.get("averaged"):
        cmd.append("--averaged")
    if cfg.get("early_stop_patience", 0) > 0:
        cmd.extend(["--early-stop-patience", str(cfg["early_stop_patience"])])
    if "early_stop_target" in cfg:
        cmd.extend(["--early-stop-target", str(cfg["early_stop_target"])])
    run_cmd(cmd)
    return output


def train_ces(seed: int, cfg: dict) -> Path:
    output = WEIGHTS_DIR / "ces" / f"seed-{seed}.txt"
    log_csv = RESULTS_DIR / f"convergence_ces_seed-{seed}.csv"
    cmd = [
        "cargo", "run", "--release", "--bin", "harmonomino", "--",
        "--algorithm", "ce",
        "--seed", str(seed),
        "--iterations", str(cfg["iterations"]),
        "--n-samples", str(cfg["n_samples"]),
        "--n-elite", str(cfg["n_elite"]),
        "--initial-std-dev", str(cfg["initial_std_dev"]),
        "--std-dev-floor", str(cfg["std_dev_floor"]),
        "--sim-length", str(cfg["sim_length"]),
        "--n-weights", str(cfg["n_weights"]),
        "--averaged-runs", str(cfg["averaged_runs"]),
        "--output", str(output),
        "--log-csv", str(log_csv),
    ]
    if cfg.get("averaged"):
        cmd.append("--averaged")
    if cfg.get("early_stop_patience", 0) > 0:
        cmd.extend(["--early-stop-patience", str(cfg["early_stop_patience"])])
    if "early_stop_target" in cfg:
        cmd.extend(["--early-stop-target", str(cfg["early_stop_target"])])
    run_cmd(cmd)
    return output


# ---------------------------------------------------------------------------
# Baselines
# ---------------------------------------------------------------------------


def create_baselines(cfg: dict, n_weights: int) -> dict[str, list[Path]]:
    baseline_dir = WEIGHTS_DIR / "baselines"
    baseline_dir.mkdir(parents=True, exist_ok=True)

    random_paths: list[Path] = []
    rng = random.Random(cfg["random_seed"])
    for i in range(cfg["random_weights"]):
        path = baseline_dir / f"random-{i:02d}.txt"
        random_paths.append(path)
        if path.exists():
            continue
        weights = [rng.uniform(-1.0, 1.0) for _ in range(n_weights)]
        write_weights(path, weights)

    return {
        "random": random_paths,
    }


# ---------------------------------------------------------------------------
# Evaluation
# ---------------------------------------------------------------------------


def run_eval(tag: str, weight_paths: list[Path], eval_cfg: dict, n_weights: int) -> None:
    if not weight_paths:
        return
    output_csv = RESULTS_DIR / f"eval_{tag}.csv"
    seeds_csv = ",".join(str(seed) for seed in eval_cfg["seeds"])
    cmd = [
        "cargo", "run", "--release", "--bin", "benchmark", "--",
        "--eval",
        "--sim-length", str(eval_cfg["sim_length"]),
        "--n-weights", str(n_weights),
        "--output-csv", str(output_csv),
        "--seeds", seeds_csv,
    ]
    for path in weight_paths:
        cmd.extend(["--weights", str(path)])
    run_cmd(cmd)


# ---------------------------------------------------------------------------
# Parameter sweeps
# ---------------------------------------------------------------------------


def run_sweep(param: str, cfg: dict) -> None:
    csv_name = f"benchmark_{param.replace('-', '_')}.csv"
    output_in_results = ROOT / "results" / csv_name
    dest = RESULTS_DIR / csv_name

    if dest.exists():
        print(f"Skipping sweep {param} (output exists).")
        return

    cmd = [
        "cargo", "run", "--release", "--bin", "benchmark", "--",
        "--sweep", param,
        "--sim-length", str(cfg["sim_length"]),
        "--n-weights", str(cfg["n_weights"]),
    ]
    run_cmd(cmd)

    # benchmark writes to <ROOT>/results/; move into experiments/results/
    if output_in_results.exists():
        shutil.move(str(output_in_results), str(dest))


# ---------------------------------------------------------------------------
# Mass optimize
# ---------------------------------------------------------------------------


def run_mass_optimize(cfg: dict) -> None:
    dest = RESULTS_DIR / "optimized_weights.csv"
    output_in_results = ROOT / "results" / "optimized_weights.csv"

    if dest.exists():
        print("Skipping mass-optimize (output exists).")
        return

    cmd = [
        "cargo", "run", "--release", "--bin", "benchmark", "--",
        "--mass-optimize", str(cfg["count"]),
        "--sim-length", str(cfg["sim_length"]),
        "--n-weights", str(cfg["n_weights"]),
    ]
    run_cmd(cmd)

    if output_in_results.exists():
        shutil.move(str(output_in_results), str(dest))


# ---------------------------------------------------------------------------
# Consistency test
# ---------------------------------------------------------------------------


def run_consistency_test(weight_path: Path, cfg: dict, n_weights: int) -> None:
    dest = RESULTS_DIR / "consistency.csv"
    if dest.exists():
        print("Skipping consistency test (output exists).")
        return

    seed = cfg["seed"]
    game_lengths: list[int] = cfg["game_lengths"]

    rows: list[tuple[int, int]] = []
    for length in game_lengths:
        tmp_csv = RESULTS_DIR / f"_consistency_tmp_{length}.csv"
        cmd = [
            "cargo", "run", "--release", "--bin", "benchmark", "--",
            "--eval",
            "--sim-length", str(length),
            "--n-weights", str(n_weights),
            "--output-csv", str(tmp_csv),
            "--seeds", str(seed),
            "--weights", str(weight_path),
        ]
        run_cmd(cmd)

        with tmp_csv.open("r", newline="") as f:
            reader = csv.DictReader(f)
            for row in reader:
                rows.append((length, int(row["rows_cleared"])))
        tmp_csv.unlink()

    with dest.open("w", newline="") as f:
        writer = csv.writer(f)
        for game_length, score in rows:
            writer.writerow([game_length, score])


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def main() -> None:
    cfg = load_config()
    ensure_dirs()

    train_cfg = cfg["training"]
    eval_cfg = cfg["evaluation"]
    hsa_cfg = cfg["hsa"]
    ces_cfg = cfg["ces"]
    baseline_cfg = cfg["baselines"]
    sweep_cfg = cfg.get("sweeps", {})
    mass_cfg = cfg.get("mass_optimize", {})
    consistency_cfg = cfg.get("consistency", {})

    # --- Phase 1: Training ---
    hsa_weights = [train_hsa(seed, hsa_cfg) for seed in train_cfg["seeds"]]
    ces_seeds = ces_cfg.get("seeds", train_cfg["seeds"])
    ces_weights = [train_ces(seed, ces_cfg) for seed in ces_seeds]

    # --- Phase 2: Baselines ---
    baselines = create_baselines(baseline_cfg, hsa_cfg["n_weights"])

    # --- Phase 3: Evaluation ---
    run_eval("hsa", hsa_weights, eval_cfg, hsa_cfg["n_weights"])
    run_eval("ces", ces_weights, eval_cfg, ces_cfg["n_weights"])
    run_eval("random", baselines["random"], eval_cfg, hsa_cfg["n_weights"])

    # --- Phase 4: Parameter sweeps ---
    if sweep_cfg:
        for param in ["bandwidth", "iterations", "pitch-adj-rate"]:
            run_sweep(param, sweep_cfg)

    # --- Phase 5: Mass optimize (for weight analysis) ---
    if mass_cfg:
        run_mass_optimize(mass_cfg)

    # --- Phase 6: Consistency test ---
    if consistency_cfg and hsa_weights:
        run_consistency_test(hsa_weights[0], consistency_cfg, hsa_cfg["n_weights"])


if __name__ == "__main__":
    main()
