import csv
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

WEIGHT_COLS = [f"w{i}" for i in range(1, 17)]


def load_config() -> dict:
    with CONFIG_PATH.open("rb") as f:
        return tomllib.load(f)


# ---------------------------------------------------------------------------
# Data loaders
# ---------------------------------------------------------------------------


def load_eval_data() -> dict[str, list[int]]:
    data: dict[str, list[int]] = {}
    for path in sorted(RESULTS_DIR.glob("eval_*.csv")):
        method = path.stem.replace("eval_", "")
        rows: list[int] = []
        with path.open("r", newline="") as f:
            reader = csv.DictReader(f)
            for row in reader:
                rows.append(int(row["rows_cleared"]))
        if rows:
            data[method] = rows
    return data


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
    summary_path = RESULTS_DIR / "summary.csv"
    with summary_path.open("w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["method", "n", "mean", "median", "std", "ci95"])
        for method, values in sorted(data.items()):
            arr = np.array(values, dtype=float)
            n = len(arr)
            mean = float(arr.mean())
            median = float(np.median(arr))
            std = float(arr.std(ddof=1)) if n > 1 else 0.0
            ci95 = float(1.96 * std / np.sqrt(n)) if n > 1 else 0.0
            writer.writerow([method, n, f"{mean:.3f}", f"{median:.3f}", f"{std:.3f}", f"{ci95:.3f}"])


# ---------------------------------------------------------------------------
# Original plots
# ---------------------------------------------------------------------------


def plot_distributions(data: dict[str, list[int]], plots_dir: Path) -> None:
    if not data:
        return
    labels = list(sorted(data.keys()))
    values = [data[label] for label in labels]

    plt.figure(figsize=(8, 4))
    plt.boxplot(values, labels=labels, showmeans=True)
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

    axes[1].set_xlabel("Weight index")
    for ax in axes:
        ax.set_ylabel("Weight value")
        ax.set_xticks(range(1, n_weights + 1))

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

    plt.figure(figsize=(12, 6))
    plt.bar(cols, means, yerr=stds, capsize=5, color="skyblue", edgecolor="navy", alpha=0.8)
    plt.xlabel("Weight Columns")
    plt.ylabel("Average Value")
    plt.title("Mean and Standard Deviation for w1 through w16")
    plt.grid(axis="y", linestyle="--", alpha=0.7)
    plt.tight_layout()
    plt.savefig(plots_dir / "weight_mean_std.pdf")
    plt.close()

    stats_summary = pd.DataFrame({"Mean": means, "Std Dev": stds})
    stats_summary.to_csv(RESULTS_DIR / "weight_stats.csv")


def plot_correlation_heatmap(df: pd.DataFrame, plots_dir: Path) -> None:
    corr = df.corr(numeric_only=True)

    sns.set_theme(style="white")
    plt.figure(figsize=(10, 8))
    sns.heatmap(corr, cmap="coolwarm", vmin=-1, vmax=1, center=0, square=True, annot=False)
    plt.title("Weight Correlation Matrix")
    plt.tight_layout()
    plt.savefig(plots_dir / "weight_correlation.pdf")
    plt.close()


def plot_weight_histograms(df: pd.DataFrame, plots_dir: Path) -> None:
    cols = [c for c in WEIGHT_COLS if c in df.columns]
    if not cols:
        return

    fig, axes = plt.subplots(nrows=4, ncols=4, figsize=(16, 14))
    fig.suptitle("Weight Distributions: w1 through w16", fontsize=22, fontweight="bold")

    axes_flat = axes.flatten()

    for i, col in enumerate(cols):
        if i >= len(axes_flat):
            break
        sns.histplot(df[col], kde=True, ax=axes_flat[i], color="royalblue", bins=20)
        axes_flat[i].set_title(f"Distribution of {col}", fontsize=12)
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
# Main
# ---------------------------------------------------------------------------


def main() -> None:
    cfg = load_config()
    plots_dir = Path(cfg.get("plots", {}).get("output_dir", "plots"))
    if not plots_dir.is_absolute():
        plots_dir = BASE_DIR / plots_dir
    plots_dir.mkdir(parents=True, exist_ok=True)
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)

    # --- Evaluation summary & distributions ---
    data = load_eval_data()
    if data:
        write_summary(data)
        plot_distributions(data, plots_dir)

    # --- Convergence curves ---
    plot_convergence(plots_dir)

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
        plot_correlation_heatmap(opt_df, plots_dir)
        plot_weight_histograms(opt_df, plots_dir)
        plot_pairwise_distances(opt_df, plots_dir)
        plot_pca(opt_df, plots_dir)


if __name__ == "__main__":
    main()
