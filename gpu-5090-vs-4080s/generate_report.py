"""Generate self-contained HTML report with Chart.js graphs from comparison_data.json."""

from __future__ import annotations

import json
import statistics
from pathlib import Path

ROOT = Path(__file__).resolve().parent
DATA = ROOT / "comparison_data.json"
OUT = ROOT / "report.html"


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
    return f"{p['scenario']} · {d}"


def is_maxpp(p: dict) -> bool:
    return "max++" in p["scenario"]


def main() -> None:
    data = json.loads(DATA.read_text(encoding="utf-8"))
    wow = data["wow_pairs"]
    llm = data["llm_pairs"]
    summary = data["summary"]

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
        llm_by_model.append(
            {"model": model, "5090": round(t5090, 1), "4080": round(t4080, 1), "pct": round(pct_gain(t5090, t4080), 1)}
        )

    def wow_series(pairs: list, metric5090: str, metric4080: str) -> dict:
        labels = [wow_label(p) for p in pairs]
        return {
            "labels": labels,
            "5090": [p[metric5090] for p in pairs],
            "4080": [p[metric4080] for p in pairs],
            "gain": [round(pct_gain(p[metric5090], p[metric4080]), 1) for p in pairs],
        }

    charts = {
        "wow_avg_std": wow_series(wow_std, "5090_fps", "4080_fps"),
        "wow_avg_maxpp": wow_series(wow_maxpp, "5090_fps", "4080_fps"),
        "wow_p1_std": wow_series(wow_std, "5090_p1", "4080_p1"),
        "wow_p01_std": wow_series(wow_std, "5090_p0.1", "4080_p0.1"),
        "wow_p001_std": wow_series(wow_std, "5090_p0.01", "4080_p0.01"),
        "wow_p99_std": wow_series(wow_std, "5090_p99", "4080_p99"),
        "wow_p1_maxpp": wow_series(wow_maxpp, "5090_p1", "4080_p1"),
        "wow_gain_std": {"labels": [wow_label(p) for p in wow_std], "gain": [p["pct_gain"] for p in wow_std]},
        "wow_gain_maxpp": {"labels": [wow_label(p) for p in wow_maxpp], "gain": [p["pct_gain"] for p in wow_maxpp]},
        "llm_tps": {
            "labels": [m["model"] for m in llm_by_model],
            "5090": [m["5090"] for m in llm_by_model],
            "4080": [m["4080"] for m in llm_by_model],
            "gain": [m["pct"] for m in llm_by_model],
        },
        "llm_detail": {
            "labels": [f"{short_model(r['model'])} · {r['workload']}" for r in llm],
            "5090": [r["5090_tps"] for r in llm],
            "4080": [r["4080_tps"] for r in llm],
            "gain": [r["pct_gain"] for r in llm],
        },
    }

    maxpp_median_gain = statistics.median(p["pct_gain"] for p in wow_maxpp if "chimearus" not in p["scenario"])
    wow_p1_median = statistics.median(pct_gain(p["5090_p1"], p["4080_p1"]) for p in wow_std)

    html = f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>RTX 5090 vs RTX 4080 SUPER</title>
  <script src="https://cdn.jsdelivr.net/npm/chart.js@4.4.1/dist/chart.umd.min.js"></script>
  <style>
    :root {{
      --bg: #0f1117; --surface: #1a1d27; --text: #e8eaed; --muted: #9aa0a6;
      --5090: #5b9bd5; --4080: #9b7bb8; --gain-pos: #6bbf8a; --gain-neg: #e07a7a;
      --border: #2a2f3a;
    }}
    * {{ box-sizing: border-box; }}
    body {{ font-family: system-ui, -apple-system, Segoe UI, sans-serif; background: var(--bg); color: var(--text); margin: 0; padding: 2rem; line-height: 1.5; }}
    h1 {{ font-size: 1.75rem; margin: 0 0 0.25rem; }}
    h2 {{ font-size: 1.15rem; margin: 2rem 0 0.5rem; border-bottom: 1px solid var(--border); padding-bottom: 0.35rem; }}
    p.sub {{ color: var(--muted); margin: 0 0 1.5rem; font-size: 0.95rem; }}
    .warn {{ background: #3d2e14; border: 1px solid #7a5c1e; color: #f0d090; padding: 0.75rem 1rem; border-radius: 6px; margin-bottom: 1.5rem; font-size: 0.9rem; }}
    .stats {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 0.75rem; margin-bottom: 1.5rem; }}
    .stat {{ background: var(--surface); border: 1px solid var(--border); border-radius: 8px; padding: 0.85rem 1rem; }}
    .stat .v {{ font-size: 1.4rem; font-weight: 600; }}
    .stat .l {{ color: var(--muted); font-size: 0.8rem; }}
    .chart-wrap {{ background: var(--surface); border: 1px solid var(--border); border-radius: 8px; padding: 1rem; margin-bottom: 1rem; position: relative; height: 360px; }}
    .chart-wrap.tall {{ height: 480px; }}
    .chart-wrap .cap {{ color: var(--muted); font-size: 0.8rem; margin-top: 0.5rem; }}
    table {{ width: 100%; border-collapse: collapse; font-size: 0.85rem; margin-top: 0.5rem; }}
    th, td {{ border: 1px solid var(--border); padding: 0.4rem 0.6rem; text-align: right; }}
    th {{ background: var(--surface); text-align: left; }}
    td:first-child, th:first-child {{ text-align: left; }}
    .pos {{ color: var(--gain-pos); }} .neg {{ color: var(--gain-neg); }}
  </style>
</head>
<body>
  <h1>RTX 5090 vs RTX 4080 SUPER</h1>
  <p class="sub">Core Ultra 9 285K · 32 GB DDR5-6400 · WoW CapFrameX (2026-05-17 vs 2026-05-30) · llama.cpp llmb (same dates)</p>

  <div class="warn">
    CapFrameX reports DX12 for all 5090 WoW captures but DX11 for every 4080S capture — even when comments say "dx12".
    Treat WoW as same-route/same-settings, not guaranteed same API.
  </div>

  <div class="stats">
    <div class="stat"><div class="v">+{(summary['wow_median_ratio']-1)*100:.1f}%</div><div class="l">WoW median avg FPS</div></div>
    <div class="stat"><div class="v">{wow_p1_median:+.1f}%</div><div class="l">WoW median 1% low FPS</div></div>
    <div class="stat"><div class="v">+{maxpp_median_gain:.0f}%</div><div class="l">WoW max++ median avg FPS</div></div>
    <div class="stat"><div class="v">+{(summary['llm_median_ratio']-1)*100:.1f}%</div><div class="l">LLM median GPU tok/s</div></div>
  </div>

  <h2>WoW — average FPS (normal settings)</h2>
  <div class="chart-wrap tall"><canvas id="wow_avg_std"></canvas></div>

  <h2>WoW — 1% low FPS (normal settings)</h2>
  <p class="sub">Bottom 1% of frames by FPS — worst sustained dips, not single spikes.</p>
  <div class="chart-wrap tall"><canvas id="wow_p1_std"></canvas></div>

  <h2>WoW — 0.1% low FPS (normal settings)</h2>
  <div class="chart-wrap tall"><canvas id="wow_p01_std"></canvas></div>

  <h2>WoW — 0.01% low FPS (normal settings)</h2>
  <div class="chart-wrap tall"><canvas id="wow_p001_std"></canvas></div>

  <h2>WoW — 99th percentile FPS (normal settings)</h2>
  <div class="chart-wrap tall"><canvas id="wow_p99_std"></canvas></div>

  <h2>WoW — 5090 gain % by scenario (normal)</h2>
  <div class="chart-wrap"><canvas id="wow_gain_std"></canvas></div>

  <h2>WoW — max++ average FPS</h2>
  <div class="chart-wrap"><canvas id="wow_avg_maxpp"></canvas></div>

  <h2>WoW — max++ 1% low FPS</h2>
  <div class="chart-wrap"><canvas id="wow_p1_maxpp"></canvas></div>

  <h2>WoW — max++ gain %</h2>
  <div class="chart-wrap"><canvas id="wow_gain_maxpp"></canvas></div>

  <h2>LLM — warm GPU tok/s by model</h2>
  <div class="chart-wrap"><canvas id="llm_tps"></canvas></div>

  <h2>LLM — 5090 gain % by model</h2>
  <div class="chart-wrap"><canvas id="llm_gain"></canvas></div>

  <h2>LLM — per model × workload</h2>
  <div class="chart-wrap tall"><canvas id="llm_detail"></canvas></div>

  <h2>WoW data table</h2>
  <table>
    <tr><th>Scenario</th><th>5090 avg</th><th>4080 avg</th><th>5090 1% low</th><th>4080 1% low</th><th>5090 0.1% low</th><th>4080 0.1% low</th><th>Gain avg</th><th>Gain 1% low</th></tr>
    {"".join(
        f'<tr><td>{wow_label(p)}</td>'
        f'<td>{p["5090_fps"]:.1f}</td><td>{p["4080_fps"]:.1f}</td>'
        f'<td>{p["5090_p1"]:.1f}</td><td>{p["4080_p1"]:.1f}</td>'
        f'<td>{p["5090_p0.1"]:.1f}</td><td>{p["4080_p0.1"]:.1f}</td>'
        f'<td class="{"pos" if p["pct_gain"]>0 else "neg"}">{p["pct_gain"]:+.1f}%</td>'
        f'<td class="{"pos" if pct_gain(p["5090_p1"], p["4080_p1"])>0 else "neg"}">{pct_gain(p["5090_p1"], p["4080_p1"]):+.1f}%</td></tr>'
        for p in wow
    )}
  </table>

<script>
const C5090 = '#5b9bd5';
const C4080 = '#9b7bb8';
const charts = {json.dumps(charts)};

function pairedBar(id, labels, s5090, s4080, suffix='') {{
  new Chart(document.getElementById(id), {{
    type: 'bar',
    data: {{
      labels,
      datasets: [
        {{ label: 'RTX 5090', data: s5090, backgroundColor: C5090 }},
        {{ label: 'RTX 4080 SUPER', data: s4080, backgroundColor: C4080 }},
      ],
    }},
    options: {{
      responsive: true, maintainAspectRatio: false,
      plugins: {{ legend: {{ labels: {{ color: '#e8eaed' }} }} }},
      scales: {{
        x: {{ ticks: {{ color: '#9aa0a6', maxRotation: 45, minRotation: 45, font: {{ size: 10 }} }} }},
        y: {{ ticks: {{ color: '#9aa0a6', callback: v => v + suffix }} }},
      }},
    }},
  }});
}}

function gainBar(id, labels, gain) {{
  new Chart(document.getElementById(id), {{
    type: 'bar',
    data: {{
      labels,
      datasets: [{{
        label: '5090 vs 4080S gain',
        data: gain,
        backgroundColor: gain.map(v => v >= 0 ? '#6bbf8a' : '#e07a7a'),
      }}],
    }},
    options: {{
      responsive: true, maintainAspectRatio: false,
      plugins: {{ legend: {{ display: false }} }},
      scales: {{
        x: {{ ticks: {{ color: '#9aa0a6', maxRotation: 45, minRotation: 45, font: {{ size: 10 }} }} }},
        y: {{ ticks: {{ color: '#9aa0a6', callback: v => v + '%' }} }},
      }},
    }},
  }});
}}

pairedBar('wow_avg_std', charts.wow_avg_std.labels, charts.wow_avg_std['5090'], charts.wow_avg_std['4080'], ' fps');
pairedBar('wow_p1_std', charts.wow_p1_std.labels, charts.wow_p1_std['5090'], charts.wow_p1_std['4080'], ' fps');
pairedBar('wow_p01_std', charts.wow_p01_std.labels, charts.wow_p01_std['5090'], charts.wow_p01_std['4080'], ' fps');
pairedBar('wow_p001_std', charts.wow_p001_std.labels, charts.wow_p001_std['5090'], charts.wow_p001_std['4080'], ' fps');
pairedBar('wow_p99_std', charts.wow_p99_std.labels, charts.wow_p99_std['5090'], charts.wow_p99_std['4080'], ' fps');
gainBar('wow_gain_std', charts.wow_gain_std.labels, charts.wow_gain_std.gain);
pairedBar('wow_avg_maxpp', charts.wow_avg_maxpp.labels, charts.wow_avg_maxpp['5090'], charts.wow_avg_maxpp['4080'], ' fps');
pairedBar('wow_p1_maxpp', charts.wow_p1_maxpp.labels, charts.wow_p1_maxpp['5090'], charts.wow_p1_maxpp['4080'], ' fps');
gainBar('wow_gain_maxpp', charts.wow_gain_maxpp.labels, charts.wow_gain_maxpp.gain);
pairedBar('llm_tps', charts.llm_tps.labels, charts.llm_tps['5090'], charts.llm_tps['4080'], ' tok/s');
gainBar('llm_gain', charts.llm_tps.labels, charts.llm_tps.gain);
pairedBar('llm_detail', charts.llm_detail.labels, charts.llm_detail['5090'], charts.llm_detail['4080'], ' tok/s');
</script>
</body>
</html>
"""

    OUT.write_text(html, encoding="utf-8")
    print(f"Wrote {OUT}")


if __name__ == "__main__":
    main()
