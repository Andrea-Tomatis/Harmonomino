import csv
import random
import shutil
import subprocess
import tomllib
from pathlib import Path

from cache import Manifest, binary_hash, cleanup_orphans, compute_expected_files

ROOT = Path(__file__).resolve().parents[1]
BASE_DIR = Path(__file__).resolve().parent
CONFIG_PATH = BASE_DIR / "config.toml"
RESULTS_DIR = BASE_DIR / "results"
WEIGHTS_DIR = BASE_DIR / "weights"


def load_config() -> dict:
    with CONFIG_PATH.open("rb") as f:
        return tomllib.load(f)


def ensure_dirs() -> None:
    for path in [RESULTS_DIR, WEIGHTS_DIR / "hsa", WEIGHTS_DIR / "ces", WEIGHTS_DIR / "baselines"]:
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


def train_hsa(seed: int, cfg: dict, manifest: Manifest) -> Path:
    output = WEIGHTS_DIR / "hsa" / f"seed-{seed}.txt"
    output_rel = f"weights/hsa/seed-{seed}.txt"
    log_rel = f"results/convergence_hsa_seed-{seed}.csv"
    h = manifest.config_hash(seed=seed, hsa=cfg)

    if manifest.is_fresh(output_rel, h) and manifest.is_fresh(log_rel, h):
        print(f"Skipping HSA seed {seed} (config unchanged).")
        return output

    log_csv = RESULTS_DIR / f"convergence_hsa_seed-{seed}.csv"
    cmd = [
        "cargo",
        "run",
        "--release",
        "--bin",
        "harmonomino",
        "--",
        "--algorithm",
        "hsa",
        "--seed",
        str(seed),
        "--iterations",
        str(cfg["iterations"]),
        "--memory-size",
        str(cfg["memory_size"]),
        "--accept-rate",
        str(cfg["accept_rate"]),
        "--pitch-adj-rate",
        str(cfg["pitch_adj_rate"]),
        "--bandwidth",
        str(cfg["bandwidth"]),
        "--sim-length",
        str(cfg["sim_length"]),
        "--n-weights",
        str(cfg["n_weights"]),
        "--averaged-runs",
        str(cfg["averaged_runs"]),
        "--output",
        str(output),
        "--log-csv",
        str(log_csv),
    ]
    if cfg.get("averaged"):
        cmd.append("--averaged")
    if cfg.get("early_stop_patience", 0) > 0:
        cmd.extend(["--early-stop-patience", str(cfg["early_stop_patience"])])
    if "early_stop_target" in cfg:
        cmd.extend(["--early-stop-target", str(cfg["early_stop_target"])])
    run_cmd(cmd)

    manifest.record(output_rel, h)
    manifest.record(log_rel, h)
    return output


def train_ces(seed: int, cfg: dict, manifest: Manifest) -> Path:
    output = WEIGHTS_DIR / "ces" / f"seed-{seed}.txt"
    output_rel = f"weights/ces/seed-{seed}.txt"
    log_rel = f"results/convergence_ces_seed-{seed}.csv"
    h = manifest.config_hash(seed=seed, ces=cfg)

    if manifest.is_fresh(output_rel, h) and manifest.is_fresh(log_rel, h):
        print(f"Skipping CES seed {seed} (config unchanged).")
        return output

    log_csv = RESULTS_DIR / f"convergence_ces_seed-{seed}.csv"
    cmd = [
        "cargo",
        "run",
        "--release",
        "--bin",
        "harmonomino",
        "--",
        "--algorithm",
        "ce",
        "--seed",
        str(seed),
        "--iterations",
        str(cfg["iterations"]),
        "--n-samples",
        str(cfg["n_samples"]),
        "--n-elite",
        str(cfg["n_elite"]),
        "--initial-std-dev",
        str(cfg["initial_std_dev"]),
        "--std-dev-floor",
        str(cfg["std_dev_floor"]),
        "--sim-length",
        str(cfg["sim_length"]),
        "--n-weights",
        str(cfg["n_weights"]),
        "--averaged-runs",
        str(cfg["averaged_runs"]),
        "--output",
        str(output),
        "--log-csv",
        str(log_csv),
    ]
    if cfg.get("averaged"):
        cmd.append("--averaged")
    if cfg.get("early_stop_patience", 0) > 0:
        cmd.extend(["--early-stop-patience", str(cfg["early_stop_patience"])])
    if "early_stop_target" in cfg:
        cmd.extend(["--early-stop-target", str(cfg["early_stop_target"])])
    run_cmd(cmd)

    manifest.record(output_rel, h)
    manifest.record(log_rel, h)
    return output


# ---------------------------------------------------------------------------
# Baselines
# ---------------------------------------------------------------------------


def create_baselines(cfg: dict, n_weights: int, manifest: Manifest) -> dict[str, list[Path]]:
    baseline_dir = WEIGHTS_DIR / "baselines"
    baseline_dir.mkdir(parents=True, exist_ok=True)

    h = manifest.config_hash(baselines=cfg, n_weights=n_weights)

    random_paths: list[Path] = []
    rng = random.Random(cfg["random_seed"])
    for i in range(cfg["random_weights"]):
        path = baseline_dir / f"random-{i:02d}.txt"
        rel = f"weights/baselines/random-{i:02d}.txt"
        random_paths.append(path)

        if manifest.is_fresh(rel, h):
            # Advance RNG to stay in sync even when skipping
            for _ in range(n_weights):
                rng.uniform(-1.0, 1.0)
            continue

        weights = [rng.uniform(-1.0, 1.0) for _ in range(n_weights)]
        write_weights(path, weights)
        manifest.record(rel, h)

    return {"random": random_paths}


# ---------------------------------------------------------------------------
# Evaluation
# ---------------------------------------------------------------------------


def run_eval(tag: str, weight_paths: list[Path], eval_cfg: dict, n_weights: int, manifest: Manifest) -> None:
    if not weight_paths:
        return

    output_rel = f"results/eval_{tag}.csv"

    # Include upstream weight hashes for transitive invalidation
    upstream = {str(p.relative_to(BASE_DIR)): manifest.hash_of(str(p.relative_to(BASE_DIR))) for p in weight_paths}
    h = manifest.config_hash(evaluation=eval_cfg, n_weights=n_weights, upstream=upstream)

    if manifest.is_fresh(output_rel, h):
        print(f"Skipping eval {tag} (config unchanged).")
        return

    output_csv = RESULTS_DIR / f"eval_{tag}.csv"
    seeds_csv = ",".join(str(seed) for seed in eval_cfg["seeds"])
    cmd = [
        "cargo",
        "run",
        "--release",
        "--bin",
        "benchmark",
        "--",
        "--eval",
        "--sim-length",
        str(eval_cfg["sim_length"]),
        "--n-weights",
        str(n_weights),
        "--output-csv",
        str(output_csv),
        "--seeds",
        seeds_csv,
    ]
    for path in weight_paths:
        cmd.extend(["--weights", str(path)])
    run_cmd(cmd)

    manifest.record(output_rel, h)


# ---------------------------------------------------------------------------
# Parameter sweeps
# ---------------------------------------------------------------------------


def run_sweep(param: str, cfg: dict, manifest: Manifest) -> None:
    csv_name = f"benchmark_{param.replace('-', '_')}.csv"
    output_in_results = ROOT / "results" / csv_name
    dest = RESULTS_DIR / csv_name
    dest_rel = f"results/{csv_name}"

    h = manifest.config_hash(sweeps=cfg, param=param)

    if manifest.is_fresh(dest_rel, h):
        print(f"Skipping sweep {param} (config unchanged).")
        return

    cmd = [
        "cargo",
        "run",
        "--release",
        "--bin",
        "benchmark",
        "--",
        "--sweep",
        param,
        "--sim-length",
        str(cfg["sim_length"]),
        "--n-weights",
        str(cfg["n_weights"]),
    ]
    run_cmd(cmd)

    # benchmark writes to <ROOT>/results/; move into experiments/results/
    if output_in_results.exists():
        shutil.move(str(output_in_results), str(dest))

    manifest.record(dest_rel, h)


# ---------------------------------------------------------------------------
# Mass optimize
# ---------------------------------------------------------------------------


def run_mass_optimize(cfg: dict, manifest: Manifest) -> None:
    dest = RESULTS_DIR / "optimized_weights.csv"
    output_in_results = ROOT / "results" / "optimized_weights.csv"
    dest_rel = "results/optimized_weights.csv"

    h = manifest.config_hash(mass_optimize=cfg)

    if manifest.is_fresh(dest_rel, h):
        print("Skipping mass-optimize (config unchanged).")
        return

    cmd = [
        "cargo",
        "run",
        "--release",
        "--bin",
        "benchmark",
        "--",
        "--mass-optimize",
        str(cfg["count"]),
        "--sim-length",
        str(cfg["sim_length"]),
        "--n-weights",
        str(cfg["n_weights"]),
    ]
    run_cmd(cmd)

    if output_in_results.exists():
        shutil.move(str(output_in_results), str(dest))

    manifest.record(dest_rel, h)


# ---------------------------------------------------------------------------
# Consistency test
# ---------------------------------------------------------------------------


def run_consistency_test(weight_path: Path, cfg: dict, n_weights: int, manifest: Manifest) -> None:
    dest = RESULTS_DIR / "consistency.csv"
    dest_rel = "results/consistency.csv"

    upstream_rel = str(weight_path.relative_to(BASE_DIR))
    upstream_hash = manifest.hash_of(upstream_rel)
    h = manifest.config_hash(consistency=cfg, n_weights=n_weights, upstream=upstream_hash)

    if manifest.is_fresh(dest_rel, h):
        print("Skipping consistency test (config unchanged).")
        return

    seed = cfg["seed"]
    game_lengths: list[int] = cfg["game_lengths"]

    rows: list[tuple[int, int]] = []
    for length in game_lengths:
        tmp_csv = RESULTS_DIR / f"_consistency_tmp_{length}.csv"
        cmd = [
            "cargo",
            "run",
            "--release",
            "--bin",
            "benchmark",
            "--",
            "--eval",
            "--sim-length",
            str(length),
            "--n-weights",
            str(n_weights),
            "--output-csv",
            str(tmp_csv),
            "--seeds",
            str(seed),
            "--weights",
            str(weight_path),
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

    manifest.record(dest_rel, h)


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def main() -> None:
    cfg = load_config()
    ensure_dirs()
    manifest = Manifest.load()

    # Invalidate all if Rust binary changed
    bh = binary_hash()
    if bh and manifest.binary_hash != bh:
        print("Binary source changed — invalidating all results.")
        manifest.entries.clear()
    manifest.binary_hash = bh

    # Remove orphaned files from previous config
    cleanup_orphans(manifest, compute_expected_files(cfg))

    train_cfg = cfg["training"]
    eval_cfg = cfg["evaluation"]
    hsa_cfg = cfg["hsa"]
    ces_cfg = cfg["ces"]
    baseline_cfg = cfg["baselines"]
    sweep_cfg = cfg.get("sweeps", {})
    mass_cfg = cfg.get("mass_optimize", {})
    consistency_cfg = cfg.get("consistency", {})

    # --- Phase 1: Training ---
    hsa_weights = [train_hsa(seed, hsa_cfg, manifest) for seed in train_cfg["seeds"]]
    ces_seeds = ces_cfg.get("seeds", train_cfg["seeds"])
    ces_weights = [train_ces(seed, ces_cfg, manifest) for seed in ces_seeds]
    manifest.save()

    # --- Phase 2: Baselines ---
    baselines = create_baselines(baseline_cfg, hsa_cfg["n_weights"], manifest)
    manifest.save()

    # --- Phase 3: Evaluation ---
    run_eval("hsa", hsa_weights, eval_cfg, hsa_cfg["n_weights"], manifest)
    run_eval("ces", ces_weights, eval_cfg, ces_cfg["n_weights"], manifest)
    run_eval("random", baselines["random"], eval_cfg, hsa_cfg["n_weights"], manifest)
    manifest.save()

    # --- Phase 4: Parameter sweeps ---
    if sweep_cfg:
        for param in ["bandwidth", "iterations", "pitch-adj-rate"]:
            run_sweep(param, sweep_cfg, manifest)
        manifest.save()

    # --- Phase 5: Mass optimize (for weight analysis) ---
    if mass_cfg:
        run_mass_optimize(mass_cfg, manifest)
        manifest.save()

    # --- Phase 6: Consistency test ---
    if consistency_cfg and hsa_weights:
        run_consistency_test(hsa_weights[0], consistency_cfg, hsa_cfg["n_weights"], manifest)
        manifest.save()


if __name__ == "__main__":
    main()
