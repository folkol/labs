import json
import os
import glob
import re
import statistics
import csv
from collections import defaultdict

CAPTURES_DIR = r"c:\Users\folkol\Documents\CapFrameX\Captures"
LLM_5090 = r"c:\Users\folkol\CursorProjects\llm-inference-benchmark\reports\windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_5090__20260530T203552\results.csv"
LLM_4080 = r"c:\Users\folkol\CursorProjects\llm-inference-benchmark\reports\windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_4080_SUPER__20260517T113755\results.csv"
OUT = r"c:\Users\folkol\CursorProjects\4080s_vs_5090\comparison_data.json"


def percentile(data, p):
    if not data:
        return None
    s = sorted(data)
    k = (len(s) - 1) * p / 100
    f = int(k)
    c = min(f + 1, len(s) - 1)
    return s[f] if f == c else s[f] + (s[c] - s[f]) * (k - f)


def compute_stats(run):
    cd = run.get("CaptureData", {})
    ft = cd.get("MsBetweenPresents") or cd.get("Frametime") or []
    if not ft:
        return None
    fps = [1000 / x for x in ft if x > 0]
    return {
        "avg_fps": sum(fps) / len(fps),
        "p0.01_fps": percentile(fps, 0.01),
        "p0.1_fps": percentile(fps, 0.1),
        "p1_fps": percentile(fps, 1),
        "p99_fps": percentile(fps, 99),
        "avg_frametime_ms": sum(ft) / len(ft),
        "p99_frametime_ms": percentile(ft, 99),
    }


def load_captures(gpu_filter=None):
    recs = []
    for f in glob.glob(os.path.join(CAPTURES_DIR, "*.json")):
        with open(f, "r", encoding="utf-8-sig") as fh:
            data = json.load(fh)
        info = data["Info"]
        gpu = info.get("GPU", "")
        comment = info.get("Comment") or ""
        if not comment.strip():
            continue
        if gpu_filter and gpu_filter not in gpu:
            continue
        stats = [compute_stats(r) for r in data.get("Runs", [])]
        stats = [s for s in stats if s]
        if not stats:
            continue
        agg = {k: statistics.mean(s[k] for s in stats) for k in stats[0]}
        recs.append(
            {
                "comment": comment,
                "gpu": gpu,
                "api": info.get("ApiInfo", ""),
                "date": info.get("CreationDate", ""),
                "stats": agg,
                "n": len(stats),
            }
        )
    return recs


def norm_comment(c):
    c = c.lower().strip()
    c = re.sub(r"\s+", " ", c)
    c = c.replace("chimaerus", "chimearus")
    c = c.replace("chimearus_20", "chimearus").replace("chimearus_21", "chimearus")
    c = c.replace("max(-dist)", "max (-dist)").replace("min(-dist)", "min (-dist)")
    c = c.replace(", ", " ").replace(",", " ")
    c = re.sub(r"\s+", " ", c)
    c = c.replace("low res all", "low").replace("native res all", "native")
    c = c.replace("all min except drawing distance", "min (-dist)")
    c = c.replace("all max except drawing distance", "max (-dist)")
    c = c.replace("all min", "min").replace("all max", "max")
    return c.strip()


def group_key(c):
    direction = "RE->SoL" if "re->sol" in c.lower() else "SoL->RE" if "sol->re" in c.lower() else None
    base = norm_comment(c)
    base = re.sub(r"\s*(re->sol|sol->re)\s*", " ", base).strip()
    return base, direction


def aggregate_by_comment(recs):
    groups = defaultdict(list)
    for r in recs:
        groups[r["comment"]].append(r)
    out = {}
    for comment, items in groups.items():
        out[comment] = {
            "n": len(items),
            "avg_fps": statistics.mean(x["stats"]["avg_fps"] for x in items),
            "p0.01_fps": statistics.mean(x["stats"]["p0.01_fps"] for x in items),
            "p0.1_fps": statistics.mean(x["stats"]["p0.1_fps"] for x in items),
            "p1_fps": statistics.mean(x["stats"]["p1_fps"] for x in items),
            "p99_fps": statistics.mean(x["stats"]["p99_fps"] for x in items),
            "avg_frametime_ms": statistics.mean(x["stats"]["avg_frametime_ms"] for x in items),
            "p99_frametime_ms": statistics.mean(x["stats"]["p99_frametime_ms"] for x in items),
            "api": items[0]["api"],
        }
    return out


def load_llm_gpu(path):
    rows = []
    with open(path, newline="") as f:
        for row in csv.DictReader(f):
            if row["device"] == "gpu":
                rows.append(row)
    return rows


def main():
    r5090 = load_captures("5090")
    r4080 = load_captures("4080")
    g5090 = aggregate_by_comment(r5090)
    g4080 = aggregate_by_comment(r4080)

    lookup4080 = {}
    for comment, data in g4080.items():
        if "dx12" in comment.lower() or comment.lower().startswith(("raid", "max++")):
            base, direction = group_key(comment)
            lookup4080[(base, direction)] = (comment, data)

    pairs = []
    unmatched5090 = []
    for c5090, d5090 in sorted(g5090.items()):
        base, direction = group_key(c5090)
        key = (base, direction)
        if key in lookup4080:
            c4080, d4080 = lookup4080[key]
            ratio = d5090["avg_fps"] / d4080["avg_fps"]
            pairs.append(
                {
                    "scenario": base,
                    "direction": direction,
                    "5090_comment": c5090,
                    "4080_comment": c4080,
                    "5090_fps": round(d5090["avg_fps"], 1),
                    "4080_fps": round(d4080["avg_fps"], 1),
                    "5090_p0.01": round(d5090["p0.01_fps"], 1),
                    "4080_p0.01": round(d4080["p0.01_fps"], 1),
                    "5090_p0.1": round(d5090["p0.1_fps"], 1),
                    "4080_p0.1": round(d4080["p0.1_fps"], 1),
                    "5090_p1": round(d5090["p1_fps"], 1),
                    "4080_p1": round(d4080["p1_fps"], 1),
                    "5090_p99": round(d5090["p99_fps"], 1),
                    "4080_p99": round(d4080["p99_fps"], 1),
                    "5090_n": d5090["n"],
                    "4080_n": d4080["n"],
                    "5090_api": d5090["api"],
                    "4080_api": d4080["api"],
                    "ratio": round(ratio, 3),
                    "pct_gain": round((ratio - 1) * 100, 1),
                }
            )
        else:
            unmatched5090.append({"comment": c5090, "norm": base, "direction": direction})

    llm5090 = {(r["model"], r["workload"]): r for r in load_llm_gpu(LLM_5090)}
    llm4080 = {(r["model"], r["workload"]): r for r in load_llm_gpu(LLM_4080)}
    llm_pairs = []
    for key in sorted(llm5090.keys()):
        if key not in llm4080:
            continue
        a = float(llm5090[key]["warm_tokens_per_sec_mean"])
        b = float(llm4080[key]["warm_tokens_per_sec_mean"])
        ratio = a / b
        llm_pairs.append(
            {
                "model": key[0],
                "workload": key[1],
                "5090_tps": round(a, 1),
                "4080_tps": round(b, 1),
                "ratio": round(ratio, 3),
                "pct_gain": round((ratio - 1) * 100, 1),
                "5090_cold_tps": round(float(llm5090[key]["cold_tokens_per_sec_mean"]), 1),
                "4080_cold_tps": round(float(llm4080[key]["cold_tokens_per_sec_mean"]), 1),
            }
        )

    benchmark_pairs = [p for p in pairs if not any(x in p["scenario"] for x in ("chimearus", "raid"))]
    out = {
        "wow_pairs": pairs,
        "llm_pairs": llm_pairs,
        "unmatched5090": unmatched5090,
        "summary": {
            "wow_median_ratio": round(statistics.median(p["ratio"] for p in benchmark_pairs), 3),
            "wow_mean_ratio": round(statistics.mean(p["ratio"] for p in benchmark_pairs), 3),
            "llm_median_ratio": round(statistics.median(p["ratio"] for p in llm_pairs), 3),
            "llm_mean_ratio": round(statistics.mean(p["ratio"] for p in llm_pairs), 3),
        },
    }
    with open(OUT, "w") as f:
        json.dump(out, f, indent=2)

    print("Matched WoW scenarios:", len(pairs))
    for p in pairs:
        print(f"  {p['scenario']} [{p['direction']}]: 5090={p['5090_fps']} vs 4080S={p['4080_fps']} ({p['pct_gain']:+.1f}%)")
    print("Unmatched 5090:", len(unmatched5090))
    print("LLM pairs:", len(llm_pairs))
    print("Summary:", out["summary"])


if __name__ == "__main__":
    main()
