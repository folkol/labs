"""Render comparison charts as PNG images for README embedding."""

from __future__ import annotations

import json
import statistics
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

ROOT = Path(__file__).resolve().parent
DATA = ROOT / "comparison_data.json"
OUT_DIR = ROOT / "charts"

C5090 = "#5b9bd5"
C4080 = "#9b7bb8"
C_GAIN_POS = "#6bbf8a"
C_GAIN_NEG = "#e07a7a"
BG = "#0f1117"
SURFACE = "#1a1d27"
TEXT = "#e8eaed"
MUTED = "#9aa0a6"


def pct_gain(a: float, b: float) -> float:
    return (a / b - 1) * 100 if b else 0.0


def short_model(name: str) -> str:
    return (
        name.replace("-Q4_K_M", "")
        .replace("-instruct", "")
        .replace("Gemma-3-4B-IT", "Gemma-3-4B")
        .replace("Phi-4-mini-instruct", "Phi-4-mini")
    )


def wow_label(p: dict) -> str:
    d = p.get("direction") or "—"
    return f"{p['scenario']}\n{d}"


def is_maxpp(p: dict) -> bool:
    return "max++" in p["scenario"]


def setup_style() -> None:
    plt.rcParams.update(
        {
            "figure.facecolor": BG,
            "axes.facecolor": SURFACE,
            "axes.edgecolor": "#2a2f3a",
            "axes.labelcolor": TEXT,
            "text.color": TEXT,
            "xtick.color": MUTED,
            "ytick.color": MUTED,
            "font.size": 10,
            "figure.dpi": 120,
        }
    )


def save(fig: plt.Figure, name: str) -> Path:
    OUT_DIR.mkdir(exist_ok=True)
    path = OUT_DIR / f"{name}.png"
    fig.savefig(path, bbox_inches="tight", facecolor=BG, edgecolor="none")
    plt.close(fig)
    return path


def paired_bar(
    filename: str,
    title: str,
    labels: list[str],
    s5090: list[float],
    s4080: list[float],
    suffix: str = "",
    height: float = 0.45,
) -> Path:
    x = np.arange(len(labels))
    w = height
    fig_h = max(4.5, 0.35 * len(labels) + 2.5)
    fig, ax = plt.subplots(figsize=(11, fig_h))
    ax.barh(x - w / 2, s5090, w, label="RTX 5090", color=C5090)
    ax.barh(x + w / 2, s4080, w, label="RTX 4080 SUPER", color=C4080)
    ax.set_yticks(x)
    ax.set_yticklabels(labels, fontsize=8)
    ax.invert_yaxis()
    ax.set_title(title, fontsize=12, pad=12)
    ax.set_xlabel(suffix.strip() or "Value")
    ax.legend(loc="lower right", frameon=False)
    ax.grid(axis="x", color="#2a2f3a", alpha=0.6)
    return save(fig, filename)


def gain_bar(filename: str, title: str, labels: list[str], gain: list[float]) -> Path:
    x = np.arange(len(labels))
    colors = [C_GAIN_POS if g >= 0 else C_GAIN_NEG for g in gain]
    fig_h = max(4.0, 0.32 * len(labels) + 2.0)
    fig, ax = plt.subplots(figsize=(10, fig_h))
    ax.barh(x, gain, color=colors, height=0.6)
    ax.set_yticks(x)
    ax.set_yticklabels(labels, fontsize=8)
    ax.invert_yaxis()
    ax.axvline(0, color=MUTED, linewidth=0.8)
    ax.set_title(title, fontsize=12, pad=12)
    ax.set_xlabel("5090 vs 4080S gain (%)")
    ax.grid(axis="x", color="#2a2f3a", alpha=0.6)
    return save(fig, filename)


def wow_series(pairs: list, m5090: str, m4080: str) -> tuple[list[str], list[float], list[float]]:
    labels = [wow_label(p) for p in pairs]
    return labels, [p[m5090] for p in pairs], [p[m4080] for p in pairs]


def main() -> None:
    setup_style()
    data = json.loads(DATA.read_text(encoding="utf-8"))
    wow = data["wow_pairs"]
    llm = data["llm_pairs"]

    wow_std = [p for p in wow if not is_maxpp(p)]
    wow_maxpp = [p for p in wow if is_maxpp(p)]

    llm_models: dict[str, list] = {}
    for row in llm:
        m = short_model(row["model"])
        llm_models.setdefault(m, []).append(row)

    llm_by_model = []
    for model, rows in sorted(llm_models.items(), key=lambda x: -statistics.mean(r["5090_tps"] for r in x[1])):
        t5090 = statistics.mean(r["5090_tps"] for r in rows)
        t4080 = statistics.mean(r["4080_tps"] for r in rows)
        llm_by_model.append((model, t5090, t4080, pct_gain(t5090, t4080)))

    written: list[Path] = []

    specs = [
        ("wow-avg-normal", "WoW average FPS — normal settings", wow_std, "5090_fps", "4080_fps", " fps"),
        ("wow-p1-normal", "WoW 1% low FPS — normal settings", wow_std, "5090_p1", "4080_p1", " fps"),
        ("wow-p01-normal", "WoW 0.1% low FPS — normal settings", wow_std, "5090_p0.1", "4080_p0.1", " fps"),
        ("wow-p001-normal", "WoW 0.01% low FPS — normal settings", wow_std, "5090_p0.01", "4080_p0.01", " fps"),
        ("wow-p99-normal", "WoW 99th percentile FPS — normal settings", wow_std, "5090_p99", "4080_p99", " fps"),
        ("wow-avg-maxpp", "WoW average FPS — max++", wow_maxpp, "5090_fps", "4080_fps", " fps"),
        ("wow-p1-maxpp", "WoW 1% low FPS — max++", wow_maxpp, "5090_p1", "4080_p1", " fps"),
    ]
    for fname, title, pairs, k5090, k4080, suffix in specs:
        labels, a, b = wow_series(pairs, k5090, k4080)
        written.append(paired_bar(fname, title, labels, a, b, suffix))

    written.append(
        gain_bar(
            "wow-gain-normal",
            "WoW 5090 gain % — normal settings",
            [wow_label(p) for p in wow_std],
            [p["pct_gain"] for p in wow_std],
        )
    )
    written.append(
        gain_bar(
            "wow-gain-maxpp",
            "WoW 5090 gain % — max++",
            [wow_label(p) for p in wow_maxpp],
            [p["pct_gain"] for p in wow_maxpp],
        )
    )

    written.append(
        paired_bar(
            "llm-tps-by-model",
            "LLM warm GPU tok/s by model",
            [m[0] for m in llm_by_model],
            [m[1] for m in llm_by_model],
            [m[2] for m in llm_by_model],
            " tok/s",
        )
    )
    written.append(
        gain_bar(
            "llm-gain-by-model",
            "LLM 5090 gain % by model",
            [m[0] for m in llm_by_model],
            [m[3] for m in llm_by_model],
        )
    )
    written.append(
        paired_bar(
            "llm-by-workload",
            "LLM warm GPU tok/s — per model × workload",
            [f"{short_model(r['model'])}\n{r['workload']}" for r in llm],
            [r["5090_tps"] for r in llm],
            [r["4080_tps"] for r in llm],
            " tok/s",
        )
    )

    print(f"Wrote {len(written)} charts to {OUT_DIR}/")
    for p in written:
        print(f"  {p.name}")


if __name__ == "__main__":
    main()
