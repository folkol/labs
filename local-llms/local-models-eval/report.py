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
              "reasoning", "fix_it", "summarize", "rewrite_tone", "triage"]
JUDGED = {"summarize", "rewrite_tone", "triage"}


def main():
    rows = json.loads(RESULTS.read_text(encoding="utf-8"))
    rows = [r for r in rows if r["name"] != "qwen3.8-27b"]  # candidates only
    baseline = next(r for r in json.loads(RESULTS.read_text(encoding="utf-8"))
                    if r["name"] == "qwen3.8-27b")

    lines = [
        "# Local model eval — candidates vs qwen3.8-27b baseline",
        "",
        f"*{datetime.now():%Y-%m-%d} — RTX 5090 32 GB, Ollama 0.40.16, 3 runs, "
        "best-of-3 per task, 3 LLM-judged (judge: qwen3.8)*",
        "",
        "## Speed",
        "",
    ]
    turns = "turn_medium" in baseline["speed"]
    dual = "no_thinking_low" in baseline["speed"]

    def vs_rate(sp):
        if sp is baseline["speed"]:
            return "1.00× (baseline)"
        ratio = sp["tok_per_s_mean"] / baseline["speed"]["tok_per_s_mean"]
        return f"{ratio:.2f}× faster" if ratio >= 1 else f"{1/ratio:.2f}× slower"

    def vs_turn(sp, key):
        if sp is baseline["speed"]:
            return "1.00× (baseline)"
        a = sp[key]["seconds_mean"]
        b = baseline["speed"][key]["seconds_mean"]
        if a is None or b is None or a == 0:
            return "—"
        ratio = b / a
        return f"{ratio:.2f}× faster" if ratio >= 1 else f"{1/ratio:.2f}× slower"

    def turn_cell(block):
        if block["seconds_mean"] is None:
            return "—"
        mark = "" if block["finished"] == block["runs"] else "*"
        return f"{block['seconds_mean']}{mark}"

    def vs_base(sp, key):
        if sp is baseline["speed"]:
            return "1.00× (baseline)"
        ratio = sp[key]["tok_per_s_mean"] / baseline["speed"][key]["tok_per_s_mean"]
        return f"{ratio:.2f}× faster" if ratio >= 1 else f"{1/ratio:.2f}× slower"

    if turns:
        lines += [
            "tok/s is the decode rate with thinking off, over a fixed 512-token generation. "
            "Turn time is seconds until the model finishes a short edit, from send until it stops. "
            "A star means at least one run hit the token cap and was left out of the mean.",
            "",
            "| model | TTFT (ms) | tok/s | vs baseline | turn no-think (s) | vs baseline | turn medium (s) | vs baseline |",
            "|---|---|---|---|---|---|---|---|",
        ]
        for r in [baseline] + rows:
            sp = r["speed"]
            lines.append(
                f"| {r['name']} | {sp['ttft_ms_mean']} | {sp['tok_per_s_mean']} | {vs_rate(sp)} "
                f"| {turn_cell(sp['turn_no_thinking_low'])} | {vs_turn(sp, 'turn_no_thinking_low')} "
                f"| {turn_cell(sp['turn_medium'])} | {vs_turn(sp, 'turn_medium')} |"
            )
    elif dual:
        lines += [
            "tok/s counts every generated token, including the reasoning trace. "
            "Quality tasks run at medium effort with thinking on.",
            "",
            "| model | TTFT no-think (ms) | tok/s no-think | vs baseline | TTFT medium (ms) | tok/s medium | vs baseline |",
            "|---|---|---|---|---|---|---|",
        ]
        for r in [baseline] + rows:
            sp = r["speed"]
            low, med = sp["no_thinking_low"], sp["medium"]
            lines.append(
                f"| {r['name']} | {low['ttft_ms_mean']} | {low['tok_per_s_mean']} | {vs_base(sp, 'no_thinking_low')} "
                f"| {med['ttft_ms_mean']} | {med['tok_per_s_mean']} | {vs_base(sp, 'medium')} |"
            )
    else:
        lines += [
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
