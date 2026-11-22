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
              "fix_it", "summarize", "rewrite_tone", "triage",
              "ifeval", "gsm8k", "humaneval"]
JUDGED = {"summarize", "rewrite_tone", "triage"}
SAMPLED = {"ifeval", "gsm8k", "humaneval"}


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
        "More details on the machine, the marks, and the tasks: [README.md](README.md).",
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

    sample_n = next(
        (r["tasks"][tid]["details"].get("samples")
         for r in [baseline] + rows for tid in SAMPLED
         if tid in r["tasks"] and isinstance(r["tasks"][tid].get("details"), dict)
         and r["tasks"][tid]["details"].get("samples")),
        None,
    )
    lines += ["", "## Quality (0–100, judged = LLM rubric)", ""]
    if sample_n:
        lines += [
            f"Rows marked † are percent correct on {sample_n} samples of an off-the-shelf "
            "task (IFEval, GSM8K, HumanEval), with thinking off. A star is an LLM-judged score. "
            "A ! means that trial was paging VRAM.",
            "",
        ]
    header = "| task | " + " | ".join(r["name"] for r in [baseline] + rows) + " |"
    lines += [header, "|---" * (len(rows) + 2) + "|"]
    for tid in TASK_ORDER:
        cells = []
        present = False
        for r in [baseline] + rows:
            td = r["tasks"].get(tid, {})
            v = td.get("score")
            if v is None and tid not in r["tasks"]:
                cells.append("—")
                continue
            present = True
            mark = "*" if tid in JUDGED else ("†" if tid in SAMPLED else "")
            if (r.get("vram") or {}).get(tid, {}).get("spill"):
                mark += "!"
            cells.append(f"{v}{mark}" if v is not None else "—")
        if present or tid not in SAMPLED:
            lines.append(f"| {tid} | " + " | ".join(cells) + " |")

    spills = []
    for r in [baseline] + rows:
        for label, info in (r.get("vram") or {}).items():
            if info.get("spill"):
                spills.append((r["name"], label, info))
    if any(r.get("vram") for r in [baseline] + rows):
        lines += ["", "## VRAM", ""]
        if spills:
            lines.append(
                "These trials kept the bus busy and the GPU idle after the load window, "
                "so the model was paging weights. A ! on a quality score is the same warning."
            )
            lines.append("")
            for name, label, info in spills:
                bus = info["bus_mbs"] / 1000
                lines.append(
                    f"- **{name} / {label}**: couldn't fit in VRAM "
                    f"(GPU {info['sm_pct']}%, bus {bus:.1f} GB/s)"
                )
        else:
            lines.append(
                "No trial that ran long enough showed the low-GPU, high-bus pattern."
            )

    # best candidate per task
    lines += ["", "## Notes", ""]
    best = {}
    for tid in TASK_ORDER:
        if tid in SAMPLED and not any(tid in r["tasks"] for r in rows):
            continue
        vals = [(r["name"], r["tasks"].get(tid, {}).get("score")) for r in rows]
        vals = [(n, -1 if s is None else s) for n, s in vals]
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
