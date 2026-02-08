"""Manifest-based pipeline caching.

Tracks which config values produced each output file via SHA-256 hashes.
On each run, the pipeline computes the current hash, compares against
the manifest, and skips or reruns accordingly.
"""

from __future__ import annotations

import hashlib
import json
import subprocess
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent
ROOT = BASE_DIR.parent
MANIFEST_PATH = BASE_DIR / ".manifest.json"


@dataclass
class ManifestEntry:
    config_hash: str
    produced_at: str


@dataclass
class Manifest:
    version: int = 1
    binary_hash: str = ""
    entries: dict[str, ManifestEntry] = field(default_factory=dict)

    @classmethod
    def load(cls) -> Manifest:
        """Load from disk, or return empty manifest if missing/corrupt."""
        try:
            raw = json.loads(MANIFEST_PATH.read_text())
            entries = {
                k: ManifestEntry(**v) for k, v in raw.get("entries", {}).items()
            }
            return cls(
                version=raw.get("version", 1),
                binary_hash=raw.get("binary_hash", ""),
                entries=entries,
            )
        except (FileNotFoundError, json.JSONDecodeError, TypeError, KeyError):
            return cls()

    def save(self) -> None:
        """Write manifest to disk."""
        raw = {
            "version": self.version,
            "binary_hash": self.binary_hash,
            "entries": {
                k: {"config_hash": v.config_hash, "produced_at": v.produced_at}
                for k, v in sorted(self.entries.items())
            },
        }
        MANIFEST_PATH.write_text(json.dumps(raw, indent=2) + "\n")

    @staticmethod
    def config_hash(**kwargs: object) -> str:
        """SHA-256 of canonical JSON of keyword arguments, truncated to 16 hex chars."""
        blob = json.dumps(kwargs, sort_keys=True, default=str)
        return hashlib.sha256(blob.encode()).hexdigest()[:16]

    def is_fresh(self, rel_path: str, expected_hash: str) -> bool:
        """Return True if the file exists on disk AND its recorded hash matches."""
        entry = self.entries.get(rel_path)
        if entry is None or entry.config_hash != expected_hash:
            return False
        return (BASE_DIR / rel_path).exists()

    def record(self, rel_path: str, config_hash: str) -> None:
        """Record that an output was produced with the given config hash."""
        self.entries[rel_path] = ManifestEntry(
            config_hash=config_hash,
            produced_at=datetime.now(timezone.utc).isoformat(),
        )

    def hash_of(self, rel_path: str) -> str:
        """Return stored config hash for transitive dependency tracking."""
        entry = self.entries.get(rel_path)
        return entry.config_hash if entry else ""

    def remove(self, rel_path: str) -> None:
        """Remove an entry from the manifest."""
        self.entries.pop(rel_path, None)


def compute_expected_files(cfg: dict) -> set[str]:
    """Compute the set of all output file paths the current config would produce."""
    expected: set[str] = set()

    train_cfg = cfg["training"]
    hsa_cfg = cfg["hsa"]
    ces_cfg = cfg["ces"]
    baseline_cfg = cfg["baselines"]

    # HSA training outputs
    for seed in train_cfg["seeds"]:
        expected.add(f"weights/hsa/seed-{seed}.txt")
        expected.add(f"results/convergence_hsa_seed-{seed}.csv")

    # CES training outputs
    ces_seeds = ces_cfg.get("seeds", train_cfg["seeds"])
    for seed in ces_seeds:
        expected.add(f"weights/ces/seed-{seed}.txt")
        expected.add(f"results/convergence_ces_seed-{seed}.csv")

    # Baseline outputs
    for i in range(baseline_cfg["random_weights"]):
        expected.add(f"weights/baselines/random-{i:02d}.txt")

    # Evaluation outputs
    expected.add("results/eval_hsa.csv")
    expected.add("results/eval_ces.csv")
    expected.add("results/eval_random.csv")

    # Sweep outputs
    if cfg.get("sweeps"):
        for param in ["bandwidth", "iterations", "pitch-adj-rate"]:
            expected.add(f"results/benchmark_{param.replace('-', '_')}.csv")

    # Mass optimize
    if cfg.get("mass_optimize"):
        expected.add("results/optimized_weights.csv")

    # Consistency
    if cfg.get("consistency"):
        expected.add("results/consistency.csv")

    return expected


def cleanup_orphans(manifest: Manifest, expected: set[str]) -> None:
    """Delete files tracked in manifest but not in expected outputs."""
    orphans = set(manifest.entries.keys()) - expected
    # Exclude virtual markers (not real files)
    orphans = {p for p in orphans if not p.startswith("_")}
    for rel_path in sorted(orphans):
        full = BASE_DIR / rel_path
        if full.exists():
            print(f"Removing orphaned output: {rel_path}")
            full.unlink()
        manifest.remove(rel_path)


def binary_hash() -> str:
    """Hash of Rust source inputs via git log."""
    try:
        result = subprocess.run(
            ["git", "log", "-1", "--format=%H", "--", "src/", "Cargo.toml", "Cargo.lock"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=True,
        )
        return result.stdout.strip()[:16]
    except (subprocess.CalledProcessError, FileNotFoundError):
        return ""
