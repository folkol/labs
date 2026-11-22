# /// script
# requires-python = ">=3.10"
# dependencies = ["pyyaml"]
# ///
"""Generate REPORT.md from results.json."""
import json
from datetime import datetime
from pathlib import Path

HERE = Path(__file__).resolve().parent
RESULTS = HERE / "results.json"
TASK_ORDER = ["json_extract", "constraints", "math_word", "coding", "reasoning",
              "summarize", "rewrite_tone"]
JUDGED = {"summarize", "rewrite_tone"}


def main():
    rows = json.loads(RESULTS.read_text(encoding="utf-8"))
    rows = [r for r in rows if r["name"] != "qwen3.8-27b"]  # candidates only
    baseline = next(r for r in json.loads(RESULTS.read_text(encoding="utf-8"))
                    if r["name"] == "qwen3.8-27b")

    lines = [
        "# Local model eval — candidates vs qwen3.8-27b baseline",
        "",
        f"*{datetime.now():%Y-%m-%d} — RTX 5090 32 GB, Ollama 0.40.16, 3 runs, "
        "best-of-3 per task, 2 LLM-judged (judge: qwen3.8)*",
        "",
        "## Speed",
        "",
        "| model | TTFT (ms) | tok/s | speedup vs baseline |",
        "|---|---|---|---|",
    ]
    for r in [baseline] + rows:
        sp = r["speed"]
        if r is baseline:
            spstr = "1.00× (baseline)"
        else:
            ratio = sp["tok_per_s_mean"] / baseline["speed"]["tok_per_s_mean"]
            spstr = f"{ratio:.2f}× faster" if ratio >= 1 else f"{1/ratio:.2f}× slower"
        lines.append(f"| {r['name']} | {sp['ttft_ms_mean']} | {sp['tok_per_s_mean']} | {spstr} |")

    lines += ["", "## Quality (0–100, judged = LLM rubric)", ""]
    header = "| task | " + " | ".join(r["name"] for r in [baseline] + rows) + " |"
    lines += [header, "|---" * (len(rows) + 2) + "|"]
    for tid in TASK_ORDER:
        cells = []
        for r in [baseline] + rows:
            td = r["tasks"].get(tid, {})
            v = td.get("score")
            cells.append(f"{v}{'*' if tid in JUDGED else ''}" if v is not None else "—")
        lines.append(f"| {tid} | " + " | ".join(cells) + " |")

    # best candidate per task
    lines += ["", "## Notes", ""]
    best = {}
    for tid in TASK_ORDER:
        vals = [(r["name"], r["tasks"].get(tid, {}).get("score", -1)) for r in rows]
        top = max(s for _, s in vals)
        winners = [n for n, s in vals if s == top]
        best[tid] = (winners, top)
    for tid, (winners, s) in best.items():
        lines.append(f"- **{tid}**: {' / '.join(winners)} ({s})")
    lines += [
        "",
        "- **Recommendation**: **ornith-1.5-35b-a3b** — matches the 27B dense "
        "baseline on every deterministic task, loses only the two judged "
        "writing tasks (83 vs 100), and is 2.35× faster with 8.8× lower TTFT. "
        "**tiel-coder-35b-a3b** is a very close second (100 on summarize, 83 "
        "on rewrite) and the right pick if code/coding tasks matter most. "
        "**gemma-4-26b-a4b** is outclassed: constraint-following 40, and 0 on "
        "coding/reasoning (it rambles for 12k chars without committing to an answer).",
        "",
        "- **minimax-h3 excluded**: the `unsloth/MiniMax-H3-GGUF` repo is a "
        "video-generation model (image-text-to-video, stable-diffusion.cpp), not a "
        "text LLM — its GGUFs have zero metadata (`kv_count=0`) and Ollama cannot "
        "load them. Its two ~30 GB files under `models/minimax-h3/` are dead weight "
        "and can be deleted.",
    ]
    (HERE / "REPORT.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"wrote {HERE / 'REPORT.md'}")


if __name__ == "__main__":
    main()
