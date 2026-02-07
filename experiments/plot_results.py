import csv
import json
import shutil
import tomllib
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns
from scipy.spatial.distance import pdist, squareform
from sklearn.cluster import DBSCAN
from sklearn.decomposition import PCA
from sklearn.preprocessing import StandardScaler

BASE_DIR = Path(__file__).resolve().parent
RESULTS_DIR = BASE_DIR / "results"
WEIGHTS_DIR = BASE_DIR / "weights"
CONFIG_PATH = BASE_DIR / "config.toml"
_DEFAULT_REPORT_DATA_DIR = BASE_DIR / ".." / "report" / "data"


def _resolve_report_data_dir() -> Path:
    try:
        with CONFIG_PATH.open("rb") as f:
            cfg = tomllib.load(f)
        rel = cfg.get("report", {}).get("data_dir", "")
        if rel:
            return (BASE_DIR / rel).resolve()
    except (FileNotFoundError, tomllib.TOMLDecodeError):
        pass
    return _DEFAULT_REPORT_DATA_DIR


REPORT_DATA_DIR = _resolve_report_data_dir()

HIGH_VARIANCE_THRESHOLD = 0.6
LOW_VARIANCE_THRESHOLD = 0.3

WEIGHT_COLS = [f"w{i}" for i in range(1, 17)]

FEATURE_NAMES: dict[str, str] = {
    "w1": "Pile Height", "w2": "Holes", "w3": "Connected Holes",
    "w4": "Altitude Diff", "w5": "Max Well Depth", "w6": "Sum of Wells",
    "w7": "Blocks", "w8": "Weighted Blocks", "w9": "Row Transitions",
    "w10": "Col Transitions", "w11": "Highest Hole", "w12": "Blocks Above Highest",
    "w13": "Potential Rows", "w14": "Smoothness", "w15": "Row Holes",
    "w16": "Hole Depth",
}

FEATURE_CATEGORIES: dict[str, list[str]] = {
    "Height/Surface": ["w1", "w4", "w14"],
    "Holes": ["w2", "w3", "w11", "w15", "w16"],
    "Wells": ["w5", "w6"],
    "Transitions": ["w9", "w10"],
    "Blocks": ["w7", "w8", "w12"],
    "Rows": ["w13"],
}


def load_config() -> dict:
    with CONFIG_PATH.open("rb") as f:
        return tomllib.load(f)


# ---------------------------------------------------------------------------
# Data loaders
# ---------------------------------------------------------------------------


KNOWN_EVAL_METHODS = {"hsa", "ces", "random"}


def load_eval_data() -> dict[str, list[int]]:
    data: dict[str, list[int]] = {}
    for path in sorted(RESULTS_DIR.glob("eval_*.csv")):
        method = path.stem.replace("eval_", "")
        if method not in KNOWN_EVAL_METHODS:
            continue
        rows: list[int] = []
        with path.open("r", newline="") as f:
            reader = csv.DictReader(f)
            for row in reader:
                rows.append(int(row["rows_cleared"]))
        if rows:
            data[method] = rows
    return data


def load_stopping_iterations() -> dict[str, list[int]]:
    """Extract actual iteration count from each convergence CSV."""
    result: dict[str, list[int]] = {}
    for prefix in ("hsa", "ces"):
        files = sorted(RESULTS_DIR.glob(f"convergence_{prefix}_seed-*.csv"))
        stops = []
        for path in files:
            with path.open("r", newline="") as f:
                reader = csv.DictReader(f)
                last_iter = 0
                for row in reader:
                    last_iter = int(row["iteration"])
            stops.append(last_iter + 1)  # 0-indexed → count
        if stops:
            result[prefix] = stops
    return result


def load_convergence(prefix: str) -> tuple[list[int], list[float], list[float], list[float]] | None:
    files = sorted(RESULTS_DIR.glob(f"convergence_{prefix}_seed-*.csv"))
    if not files:
        return None

    buckets: dict[int, dict[str, list[float]]] = {}
    for path in files:
        with path.open("r", newline="") as f:
            reader = csv.DictReader(f)
            for row in reader:
                iteration = int(row["iteration"])
                bucket = buckets.setdefault(iteration, {"best": [], "mean": [], "worst": []})
                bucket["best"].append(float(row["best"]))
                bucket["mean"].append(float(row["mean"]))
                bucket["worst"].append(float(row["worst"]))

    iterations = sorted(buckets.keys())
    best = [float(np.mean(buckets[i]["best"])) for i in iterations]
    mean = [float(np.mean(buckets[i]["mean"])) for i in iterations]
    worst = [float(np.mean(buckets[i]["worst"])) for i in iterations]
    return iterations, best, mean, worst


def read_weights(path: Path) -> list[float]:
    values: list[float] = []
    for line in path.read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        values.append(float(line))
    return values


def load_weight_matrix(glob_pattern: str) -> list[list[float]]:
    paths = sorted(WEIGHTS_DIR.glob(glob_pattern))
    return [read_weights(path) for path in paths]


def load_sweep_csv(filename: str) -> tuple[list[str], list[float]] | None:
    path = RESULTS_DIR / filename
    if not path.exists():
        return None
    x, y = [], []
    with path.open("r") as f:
        for row in csv.reader(f):
            x.append(row[0])
            y.append(float(row[1]))
    return x, y


def load_optimized_weights() -> pd.DataFrame | None:
    path = RESULTS_DIR / "optimized_weights.csv"
    if not path.exists():
        return None
    return pd.read_csv(path)


def load_consistency() -> tuple[list[str], list[float]] | None:
    path = RESULTS_DIR / "consistency.csv"
    if not path.exists():
        return None
    x, y = [], []
    with path.open("r") as f:
        for row in csv.reader(f):
            x.append(row[0])
            y.append(float(row[1]))
    return x, y


# ---------------------------------------------------------------------------
# Summary statistics
# ---------------------------------------------------------------------------


def write_summary(data: dict[str, list[int]]) -> None:
    rows = [["method", "n", "mean", "median", "std", "ci95"]]
    for method, values in sorted(data.items()):
        arr = np.array(values, dtype=float)
        n = len(arr)
        mean = float(arr.mean())
        median = float(np.median(arr))
        std = float(arr.std(ddof=1)) if n > 1 else 0.0
        ci95 = float(1.96 * std / np.sqrt(n)) if n > 1 else 0.0
        rows.append([method, n, f"{mean:.3f}", f"{median:.3f}", f"{std:.3f}", f"{ci95:.3f}"])

    for dest in (RESULTS_DIR / "summary.csv", REPORT_DATA_DIR / "summary.csv"):
        with dest.open("w", newline="") as f:
            csv.writer(f).writerows(rows)


# ---------------------------------------------------------------------------
# Original plots
# ---------------------------------------------------------------------------


def plot_distributions(data: dict[str, list[int]], plots_dir: Path) -> None:
    if not data:
        return
    labels = list(sorted(data.keys()))
    values = [data[label] for label in labels]

    plt.figure(figsize=(8, 4))
    plt.boxplot(values, tick_labels=labels, showmeans=True)
    plt.ylabel("Rows cleared")
    plt.xticks(rotation=30, ha="right")
    plt.tight_layout()
    plt.savefig(plots_dir / "rows_cleared_distribution.pdf")
    plt.close()


def plot_convergence(plots_dir: Path) -> None:
    hsa = load_convergence("hsa")
    ces = load_convergence("ces")
    if not hsa and not ces:
        return

    plt.figure(figsize=(8, 4))

    if hsa:
        iterations, best, mean, _ = hsa
        plt.plot(iterations, best, label="HSA best")
        plt.plot(iterations, mean, label="HSA mean", linestyle="--")

    if ces:
        iterations, best, mean, _ = ces
        plt.plot(iterations, best, label="CES best")
        plt.plot(iterations, mean, label="CES mean", linestyle="--")

    plt.xlabel("Iteration")
    plt.ylabel("Fitness")
    plt.legend()
    plt.tight_layout()
    plt.savefig(plots_dir / "fitness_over_iter.pdf")
    plt.close()


def write_stopping_csv(stops: dict[str, list[int]]) -> None:
    """Write stopping_iterations.csv to results and report data directories."""
    rows = [["algorithm", "seed_index", "stopping_iteration"]]
    for algo, iters in sorted(stops.items()):
        for idx, stop in enumerate(iters):
            rows.append([algo, idx, stop])

    for dest in (RESULTS_DIR / "stopping_iterations.csv", REPORT_DATA_DIR / "stopping_iterations.csv"):
        with dest.open("w", newline="") as f:
            csv.writer(f).writerows(rows)


def plot_stopping_iterations(stops: dict[str, list[int]], cfg: dict, plots_dir: Path) -> None:
    """Grouped bar chart showing when each seed stopped iterating."""
    algo_labels = {"hsa": "HSA", "ces": "CES"}
    algo_colors = {"hsa": "#4c72b0", "ces": "#dd8452"}
    max_iters = {"hsa": cfg["hsa"]["iterations"], "ces": cfg["ces"]["iterations"]}

    max_seeds = max(len(v) for v in stops.values())
    seed_indices = np.arange(max_seeds)
    n_algos = len(stops)
    bar_width = 0.8 / n_algos

    fig, ax = plt.subplots(figsize=(10, 5))

    for i, (algo, iters) in enumerate(sorted(stops.items())):
        offsets = seed_indices[:len(iters)] + i * bar_width - (n_algos - 1) * bar_width / 2
        ax.bar(offsets, iters, width=bar_width, label=algo_labels.get(algo, algo),
               color=algo_colors.get(algo, "gray"), edgecolor="black", alpha=0.85)
        ax.axhline(max_iters[algo], color=algo_colors.get(algo, "gray"),
                   linestyle="--", linewidth=1, alpha=0.6,
                   label=f"{algo_labels.get(algo, algo)} max ({max_iters[algo]})")

    ax.set_xticks(seed_indices)
    ax.set_xticklabels([str(i) for i in range(max_seeds)])
    ax.set_xlabel("Seed index")
    ax.set_ylabel("Iteration stopped at")
    ax.set_title("Early Stopping: Iteration Count per Seed")
    ax.legend(fontsize=8)
    ax.grid(axis="y", linestyle="--", alpha=0.5)
    fig.tight_layout()
    fig.savefig(plots_dir / "stopping_iterations.pdf")
    plt.close(fig)


def plot_weight_distributions(plots_dir: Path) -> None:
    hsa = load_weight_matrix("hsa/seed-*.txt")
    ces = load_weight_matrix("ces/seed-*.txt")

    if not hsa and not ces:
        return

    n_weights = len(hsa[0]) if hsa else len(ces[0])

    fig, axes = plt.subplots(2, 1, figsize=(9, 6), sharex=True)

    if hsa:
        hsa_arr = np.array(hsa)
        axes[0].violinplot([hsa_arr[:, i] for i in range(n_weights)], showmeans=True)
        axes[0].set_title("HSA weight distributions")
    else:
        axes[0].axis("off")

    if ces:
        ces_arr = np.array(ces)
        axes[1].violinplot([ces_arr[:, i] for i in range(n_weights)], showmeans=True)
        axes[1].set_title("CES weight distributions")
    else:
        axes[1].axis("off")

    feature_labels = [FEATURE_NAMES.get(f"w{i}", f"w{i}") for i in range(1, n_weights + 1)]
    axes[1].set_xlabel("Feature")
    for ax in axes:
        ax.set_ylabel("Weight value")
        ax.set_xticks(range(1, n_weights + 1))
        ax.set_xticklabels(feature_labels, rotation=45, ha="right", fontsize=7)

    plt.tight_layout()
    plt.savefig(plots_dir / "weights_distribution.pdf")
    plt.close()


# ---------------------------------------------------------------------------
# Sweep plots (inspired by teammate notebook)
# ---------------------------------------------------------------------------


def plot_sweep(filename: str, plots_dir: Path, *, title: str, xlabel: str,
               ylabel: str = "score", reference: float = 0, padding: float = 1.0) -> None:
    result = load_sweep_csv(filename)
    if result is None:
        return
    x, y = result

    plt.figure(figsize=(8, 4))

    if reference > 0:
        plt.plot(x, [reference] * len(x), "--", color="black", label="theoretical maximum")
    if reference == -1:
        z = [int(xi) / 2.5 - 1 for xi in x]
        plt.plot(x, z, "--", color="black", label="theoretical maximum")

    plt.plot(x, y, "o", color="black", label="simulation result")

    if y:
        y_min, y_max = min(y), max(y)
        margin = (y_max - y_min) * padding
        if margin == 0:
            plt.ylim(y_min - 1, y_min + 1)
        else:
            plt.ylim(y_min - margin, y_max + margin)

    plt.xlabel(xlabel)
    plt.ylabel(ylabel)
    plt.title(title)
    plt.grid(True)
    plt.legend()
    plt.tight_layout()
    plt.savefig(plots_dir / f"{Path(filename).stem}.pdf")
    plt.close()


# ---------------------------------------------------------------------------
# Consistency test plots
# ---------------------------------------------------------------------------


def plot_consistency(plots_dir: Path) -> None:
    result = load_consistency()
    if result is None:
        return
    x, y = result

    # Score vs game length with theoretical max
    plt.figure(figsize=(8, 4))
    z = [int(xi) / 2.5 - 1 for xi in x]
    plt.plot(x, z, "--", color="black", label="theoretical maximum")
    plt.plot(x, y, "o", color="black", label="simulation result")
    plt.xlabel("game length")
    plt.ylabel("score")
    plt.title("Consistency Test")
    plt.grid(True)
    plt.legend()
    plt.tight_layout()
    plt.savefig(plots_dir / "consistency_test.pdf")
    plt.close()

    # Absolute error
    plt.figure(figsize=(8, 4))
    errors = [abs(int(x[i]) / 2.5 - 1 - y[i]) for i in range(len(x))]
    plt.plot(x, errors, "o", color="black", label="absolute error")
    plt.xlabel("game length")
    plt.ylabel("absolute error")
    plt.title("Consistency Test: Absolute Error")
    plt.grid(True)
    plt.legend()
    plt.tight_layout()
    plt.savefig(plots_dir / "consistency_error.pdf")
    plt.close()


# ---------------------------------------------------------------------------
# Weight analysis plots (from teammate notebook)
# ---------------------------------------------------------------------------


def plot_weight_mean_std(df: pd.DataFrame, plots_dir: Path) -> None:
    cols = [c for c in WEIGHT_COLS if c in df.columns]
    if not cols:
        return

    means = df[cols].mean()
    stds = df[cols].std()

    labels = [FEATURE_NAMES.get(c, c) for c in cols]

    plt.figure(figsize=(12, 6))
    plt.bar(labels, means, yerr=stds, capsize=5, color="skyblue", edgecolor="navy", alpha=0.8)
    plt.xticks(rotation=45, ha="right")
    plt.xlabel("Feature")
    plt.ylabel("Average Value")
    plt.title("Mean and Standard Deviation of Learned Weights")
    plt.grid(axis="y", linestyle="--", alpha=0.7)
    plt.tight_layout()
    plt.savefig(plots_dir / "weight_mean_std.pdf")
    plt.close()

    # Build enriched weight_stats.csv
    ranked = stds.sort_values().index.tolist()
    rows = [["weight", "feature_name", "mean", "std", "stability_rank", "high_variance"]]
    for c in cols:
        rank = ranked.index(c) + 1  # 1 = lowest std (most stable)
        hv = "true" if float(stds[c]) > HIGH_VARIANCE_THRESHOLD else "false"
        rows.append([c, FEATURE_NAMES.get(c, c), f"{means[c]:.4f}", f"{stds[c]:.4f}", rank, hv])

    for dest in (RESULTS_DIR / "weight_stats.csv", REPORT_DATA_DIR / "weight_stats.csv"):
        with dest.open("w", newline="") as f:
            csv.writer(f).writerows(rows)


def plot_correlation_heatmap(df: pd.DataFrame, plots_dir: Path) -> None:
    corr = df.corr(numeric_only=True)
    labels = [FEATURE_NAMES.get(c, c) for c in corr.columns]

    sns.set_theme(style="white")
    plt.figure(figsize=(10, 8))
    sns.heatmap(
        corr,
        xticklabels=labels,
        yticklabels=labels,
        cmap="coolwarm",
        vmin=-1,
        vmax=1,
        center=0,
        square=True,
        annot=False,
    )
    plt.xticks(rotation=45, ha="right")
    plt.yticks(rotation=0)
    plt.tight_layout()
    plt.savefig(plots_dir / "weight_correlation.pdf")
    plt.close()


def plot_weight_histograms(df: pd.DataFrame, plots_dir: Path) -> None:
    cols = [c for c in WEIGHT_COLS if c in df.columns]
    if not cols:
        return

    fig, axes = plt.subplots(nrows=4, ncols=4, figsize=(16, 14))
    fig.suptitle("Weight Distributions by Feature", fontsize=22, fontweight="bold")

    axes_flat = axes.flatten()

    for i, col in enumerate(cols):
        if i >= len(axes_flat):
            break
        label = FEATURE_NAMES.get(col, col)
        sns.histplot(df[col], kde=True, ax=axes_flat[i], color="royalblue", bins=20)
        axes_flat[i].set_title(f"{label} ({col})", fontsize=12)
        axes_flat[i].set_xlabel("")
        axes_flat[i].set_ylabel("Frequency")

    # Hide unused axes
    for j in range(len(cols), len(axes_flat)):
        axes_flat[j].axis("off")

    plt.tight_layout(rect=[0, 0.03, 1, 0.95])
    plt.savefig(plots_dir / "weight_histograms.pdf")
    plt.close()


def plot_pairwise_distances(df: pd.DataFrame, plots_dir: Path) -> None:
    cols = [c for c in WEIGHT_COLS if c in df.columns]
    if not cols or len(df) < 2:
        return

    vectors = df[cols].values
    distances = pdist(vectors, metric="euclidean")
    dist_matrix = squareform(distances)
    upper_tri = np.triu(dist_matrix)
    full_dist_matrix = upper_tri + upper_tri.T

    # --- DBSCAN cluster stability ---
    avg_dist = full_dist_matrix[full_dist_matrix > 0].mean()
    eps_values = np.linspace(avg_dist * 0.2, avg_dist, 10)
    cluster_history = []

    for eps in eps_values:
        dbscan = DBSCAN(eps=eps, min_samples=2, metric="precomputed")
        labels = dbscan.fit_predict(full_dist_matrix)
        cluster_history.append(labels)

    history_df = pd.DataFrame(cluster_history, index=np.round(eps_values, 2))

    plt.figure(figsize=(14, 8))
    sns.heatmap(history_df, cmap="viridis", cbar_kws={"label": "Cluster ID (-1 is Noise)"})
    plt.title("DBSCAN Cluster Stability: Cluster ID vs. Epsilon Value")
    plt.xlabel("Data Point Index (Run #)")
    plt.ylabel("Epsilon (eps)")
    plt.tight_layout()
    plt.savefig(plots_dir / "dbscan_stability.pdf")
    plt.close()

    # --- K-distance elbow plot ---
    nearest_neighbor_distances = np.sort(full_dist_matrix, axis=1)[:, 1]
    sorted_distances = np.sort(nearest_neighbor_distances)

    plt.figure(figsize=(8, 5))
    plt.plot(sorted_distances)
    plt.axhline(y=avg_dist / 2, color="r", linestyle="--", label="Average Distance / 2")
    plt.title("k-Distance Plot (Finding the Elbow)")
    plt.xlabel("Points sorted by distance")
    plt.ylabel("Distance to nearest neighbor (eps)")
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    plt.savefig(plots_dir / "k_distance_elbow.pdf")
    plt.close()


def plot_pca(df: pd.DataFrame, plots_dir: Path) -> None:
    cols = [c for c in WEIGHT_COLS if c in df.columns]
    if not cols or len(df) < 3:
        return

    numeric = df[cols + ["Score"]].dropna()
    scaler = StandardScaler()
    scaled = scaler.fit_transform(numeric)

    pca = PCA(n_components=2)
    pca_results = pca.fit_transform(scaled)
    pca_df = pd.DataFrame(pca_results, columns=["PC1", "PC2"])
    pca_df["Score"] = numeric["Score"].values

    plt.figure(figsize=(8, 6))
    scatter = plt.scatter(pca_df["PC1"], pca_df["PC2"], c=pca_df["Score"],
                          cmap="viridis", edgecolors="k", s=60, alpha=0.8)
    plt.colorbar(scatter, label="Score")
    plt.xlabel(f"PC1 ({pca.explained_variance_ratio_[0]:.1%} variance)")
    plt.ylabel(f"PC2 ({pca.explained_variance_ratio_[1]:.1%} variance)")
    plt.title("PCA of Optimized Weight Vectors")
    plt.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(plots_dir / "pca_weights.pdf")
    plt.close()


# ---------------------------------------------------------------------------
# Feature-category grouped plot
# ---------------------------------------------------------------------------


def plot_weight_categories(df: pd.DataFrame, plots_dir: Path) -> None:
    """Grouped bar chart of mean weights, colored and grouped by feature category."""
    cols = [c for c in WEIGHT_COLS if c in df.columns]
    if not cols:
        return

    means = df[cols].mean()
    stds = df[cols].std()

    # Build ordered list of weights grouped by category.
    ordered_weights: list[str] = []
    category_labels: list[str] = []
    colors: list[str] = []

    palette = {
        "Height/Surface": "#4c72b0",
        "Holes": "#dd8452",
        "Wells": "#55a868",
        "Transitions": "#c44e52",
        "Blocks": "#8172b3",
        "Rows": "#937860",
    }

    for cat, members in FEATURE_CATEGORIES.items():
        for w in members:
            if w in cols:
                ordered_weights.append(w)
                category_labels.append(cat)
                colors.append(palette.get(cat, "gray"))

    x_labels = [FEATURE_NAMES.get(w, w) for w in ordered_weights]
    y = [float(means[w]) for w in ordered_weights]
    yerr = [float(stds[w]) for w in ordered_weights]
    x = np.arange(len(ordered_weights))

    fig, ax = plt.subplots(figsize=(14, 5))
    ax.bar(x, y, yerr=yerr, capsize=4, color=colors, edgecolor="black", alpha=0.85)
    ax.set_xticks(x)
    ax.set_xticklabels(x_labels, rotation=45, ha="right", fontsize=8)
    ax.set_ylabel("Mean weight value")
    ax.set_title("Learned Weights by Feature Category")
    ax.axhline(0, color="black", linewidth=0.5)
    ax.grid(axis="y", linestyle="--", alpha=0.5)

    # Add category separators and legend patches.
    from matplotlib.patches import Patch
    legend_handles = []
    prev_cat = None
    for i, cat in enumerate(category_labels):
        if prev_cat is not None and cat != prev_cat:
            ax.axvline(i - 0.5, color="grey", linewidth=0.8, linestyle=":")
        if cat != prev_cat:
            legend_handles.append(Patch(facecolor=palette.get(cat, "gray"), label=cat))
        prev_cat = cat

    ax.legend(handles=legend_handles, loc="upper left", fontsize=8)
    fig.tight_layout()
    fig.savefig(plots_dir / "weight_categories.pdf")
    plt.close(fig)


# ---------------------------------------------------------------------------
# Params export
# ---------------------------------------------------------------------------


def write_params_json(cfg: dict) -> None:
    params = {
        "n_features": cfg["hsa"]["n_weights"],
        "training_seeds": len(cfg["training"]["seeds"]),
        "training_sim_length": cfg["training"]["sim_length"],
        "eval_seeds": len(cfg["evaluation"]["seeds"]),
        "eval_sim_length": cfg["evaluation"]["sim_length"],
        "hsa_memory_size": cfg["hsa"]["memory_size"],
        "hsa_iterations": cfg["hsa"]["iterations"],
        "hsa_accept_rate": cfg["hsa"]["accept_rate"],
        "hsa_pitch_adj_rate": cfg["hsa"]["pitch_adj_rate"],
        "hsa_bandwidth": cfg["hsa"]["bandwidth"],
        "ces_n_samples": cfg["ces"]["n_samples"],
        "ces_n_elite": cfg["ces"]["n_elite"],
        "ces_iterations": cfg["ces"]["iterations"],
        "ces_initial_std_dev": cfg["ces"]["initial_std_dev"],
        "ces_std_dev_floor": cfg["ces"]["std_dev_floor"],
        "ces_early_stop_target": cfg["ces"]["early_stop_target"],
        "random_weights_count": cfg["baselines"]["random_weights"],
        "mass_optimize_count": cfg["mass_optimize"]["count"],
        "high_variance_threshold": HIGH_VARIANCE_THRESHOLD,
        "low_variance_threshold": LOW_VARIANCE_THRESHOLD,
    }
    for dest in (RESULTS_DIR / "params.json", REPORT_DATA_DIR / "params.json"):
        dest.write_text(json.dumps(params, indent=2) + "\n")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def main() -> None:
    cfg = load_config()
    plots_dir = Path(cfg.get("plots", {}).get("output_dir", "plots"))
    if not plots_dir.is_absolute():
        plots_dir = BASE_DIR / plots_dir
    plots_dir.mkdir(parents=True, exist_ok=True)
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    REPORT_DATA_DIR.mkdir(parents=True, exist_ok=True)

    # --- Export experiment parameters ---
    write_params_json(cfg)

    # --- Evaluation summary & distributions ---
    data = load_eval_data()
    if data:
        write_summary(data)
        plot_distributions(data, plots_dir)

    # --- Convergence curves ---
    plot_convergence(plots_dir)

    # --- Early stopping analysis ---
    stops = load_stopping_iterations()
    if stops:
        write_stopping_csv(stops)
        plot_stopping_iterations(stops, cfg, plots_dir)

    # --- Weight violin plots ---
    plot_weight_distributions(plots_dir)

    # --- Parameter sweep plots ---
    plot_sweep("benchmark_bandwidth.csv", plots_dir,
               title="Benchmark Bandwidth", xlabel="bandwidth", reference=199, padding=100)
    plot_sweep("benchmark_iterations.csv", plots_dir,
               title="Benchmark MaxIter", xlabel="maximum number of iterations", reference=199, padding=1)
    plot_sweep("benchmark_pitch_adj_rate.csv", plots_dir,
               title="Benchmark Pitch Adjustment Rate", xlabel="rate", reference=199, padding=100)

    # --- Consistency test ---
    plot_consistency(plots_dir)

    # --- Mass-optimize weight analysis ---
    opt_df = load_optimized_weights()
    if opt_df is not None:
        plot_weight_mean_std(opt_df, plots_dir)
        plot_weight_categories(opt_df, plots_dir)
        plot_correlation_heatmap(opt_df, plots_dir)
        plot_weight_histograms(opt_df, plots_dir)
        plot_pairwise_distances(opt_df, plots_dir)
        plot_pca(opt_df, plots_dir)


if __name__ == "__main__":
    main()
