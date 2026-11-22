#!/usr/bin/env python3
"""
Regenerate README throughput tables + reports/samples/cross-machine-matrix.html
from results.json files under reports/.

For each hardware configuration (OS + CPU + GPU), picks the newest run by run_id.
Rows cover the full bench.toml matrix (six models × three workloads × CPU/GPU = 36 scenarios).
Machines whose latest run used a smaller matrix show em dashes for missing scenarios.

Run from repo root:
    python scripts/gen_cross_machine_matrix.py
"""

from __future__ import annotations

import json
import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
REPORTS_DIR = REPO_ROOT / "reports"
BENCH_TOML = REPO_ROOT / "bench.toml"
README_PATH = REPO_ROOT / "README.md"

README_BEGIN = "<!-- BEGIN cross-machine-matrix -->"
README_END = "<!-- END cross-machine-matrix -->"

RUN_ID_RE = re.compile(r"^\d{8}T\d{6}$")
REPORT_DIR_RE = re.compile(
    r"^(?P<config>.+)__(?P<run_id>\d{8}T\d{6})$"
)

SAMPLE_WORKLOADS = ["summarize", "code", "assistant"]
WORKLOAD_LABELS = {
    "summarize": "Summarization",
    "code": "Code Generation",
    "assistant": "Generic Assistant",
}

# Preferred column order for known hosts; others sort after alphabetically by config key.
COLUMN_ORDER: dict[str, int] = {
    "windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_4080_SUPER": 0,
    "linux-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_4080_SUPER": 1,
    "windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_5090": 2,
    "windows-x86_64__CPU-AMD_Ryzen_9_9950X3D_16_Core_Processor__GPU-NVIDIA_GeForce_RTX_5090": 3,
    "macos-aarch64__CPU-Apple_M4__GPU-Apple_Silicon_GPU": 4,
}


def load_bench_models(path: Path) -> list[str]:
    models: list[str] = []
    for block in re.split(r"\[\[models\]\]", path.read_text(encoding="utf-8"))[1:]:
        m = re.search(r'name\s*=\s*"([^"]+)"', block)
        if m:
            models.append(m.group(1))
    if not models:
        raise SystemExit(f"no [[models]] entries found in {path.relative_to(REPO_ROOT)}")
    return models


BENCH_MODELS = load_bench_models(BENCH_TOML)
MODEL_ORDER = {name: i for i, name in enumerate(BENCH_MODELS)}


def load_scenarios(path: Path) -> list[dict]:
    data = json.loads(path.read_text(encoding="utf-8"))
    return data["scenarios"]


def load_hardware(path: Path) -> dict:
    data = json.loads(path.read_text(encoding="utf-8"))
    return data["hardware"]


def parse_report_dir(name: str) -> tuple[str, str] | None:
    m = REPORT_DIR_RE.match(name)
    if not m or not RUN_ID_RE.match(m.group("run_id")):
        return None
    return m.group("config"), m.group("run_id")


def shorten_cpu(brand: str) -> str:
    s = (
        brand.replace("(R)", "")
        .replace("(r)", "")
        .replace("(TM)", "")
        .replace("(tm)", "")
        .replace("™", "")
        .strip()
    )
    if "Ultra" in s:
        parts = s.split()
        for i, p in enumerate(parts):
            if p == "Ultra" and i + 2 < len(parts):
                return " ".join(parts[i : i + 3])
    if "Ryzen" in s:
        parts = s.split()
        for i, p in enumerate(parts):
            if p == "Ryzen" and i + 2 < len(parts):
                return " ".join(parts[i : i + 3])
    if "Apple" in s and "M" in s:
        parts = s.split()
        for i, p in enumerate(parts):
            if p.startswith("M") and p[1:].isdigit():
                return f"Apple {p}"
    return s


def shorten_gpu(name: str) -> str:
    if name == "Apple Silicon GPU":
        return name
    for prefix in ("NVIDIA GeForce ", "AMD Radeon "):
        if name.startswith(prefix):
            return name[len(prefix) :]
    return name


def hardware_label(hw: dict) -> str:
    os_names = {"windows": "Windows", "linux": "Linux", "macos": "macOS"}
    os_name = os_names.get(hw["os"], hw["os"].title())
    cpu = shorten_cpu(hw["cpu_brand"])
    if hw.get("gpu_info"):
        gpu = shorten_gpu(hw["gpu_info"][0]["name"])
    else:
        gpu = "CPU only"
    return f"{os_name} · {cpu} · {gpu}"


def discover_latest_runs(reports_dir: Path) -> list[dict[str, str]]:
    """Return one run dict per hardware config, choosing the newest run_id."""
    by_config: dict[str, tuple[str, Path]] = {}

    for json_path in reports_dir.rglob("results.json"):
        parsed = parse_report_dir(json_path.parent.name)
        if parsed is None:
            continue
        config_key, run_id = parsed
        report_html = json_path.parent / "report.html"
        if not report_html.is_file():
            continue

        prev = by_config.get(config_key)
        if prev is None or run_id > prev[0]:
            by_config[config_key] = (run_id, json_path)

    if not by_config:
        raise SystemExit(f"no benchmark reports found under {reports_dir.relative_to(REPO_ROOT)}")

    runs: list[dict[str, str]] = []
    for config_key, (run_id, json_path) in by_config.items():
        rel_json = json_path.relative_to(REPO_ROOT).as_posix()
        rel_report = json_path.parent.joinpath("report.html").relative_to(REPO_ROOT).as_posix()
        hw = load_hardware(json_path)
        runs.append(
            {
                "id": config_key,
                "label": hardware_label(hw),
                "config_key": config_key,
                "run_id": run_id,
                "json": rel_json,
                "report": rel_report,
            }
        )

    runs.sort(key=lambda r: (COLUMN_ORDER.get(r["config_key"], 100), r["config_key"]))
    return runs


def scenario_sort_key(s: dict) -> tuple[int, int, int]:
    """All CPU scenarios first, then GPU; within device by bench.toml model order then workload."""
    dev_rank = 0 if s["device"] == "cpu" else 1
    model_rank = MODEL_ORDER.get(s["model_name"], 99)
    workload_order = {"summarize": 0, "code": 1, "assistant": 2}
    return (dev_rank, model_rank, workload_order.get(s["workload_id"], 99))


def canonical_scenarios() -> list[dict]:
    scenarios: list[dict] = []
    for device in ("cpu", "gpu"):
        for model in BENCH_MODELS:
            for wid in SAMPLE_WORKLOADS:
                scenarios.append(
                    {
                        "model_name": model,
                        "workload_id": wid,
                        "workload_label": WORKLOAD_LABELS[wid],
                        "device": device,
                    }
                )
    return sorted(scenarios, key=scenario_sort_key)


def scenario_row_markdown(s: dict) -> str:
    """Row label when CPU/GPU is implied by table section."""
    model = s["model_name"].replace("-Q4_K_M", "")
    return f"{model} · {s['workload_label']}"


def scenario_row_plain(s: dict) -> str:
    return f"{s['model_name'].replace('-Q4_K_M', '')} · {s['workload_label']} · {s['device'].upper()}"


def tok_mean(metrics: dict, key: str = "tokens_per_sec_mean") -> float | None:
    if metrics.get("success_rate", 1.0) == 0:
        return None
    v = metrics.get(key)
    return float(v) if v is not None else None


def fmt_cell(v: float | None) -> str:
    if v is None:
        return "—"
    return str(round(v))


def markdown_tables(
    runs: list[dict[str, str]],
    matrix_warm: dict,
    rows_plain: list[str],
    scenarios_ref: list[dict],
) -> str:
    lines: list[str] = []

    def emit_device_table(section: str, mat: dict[str, dict[str, float | None]], device: str) -> None:
        lines.append(f"**{section}**")
        lines.append("")
        header = "| Scenario | " + " | ".join(f"[{r['label']}]({r['report']})" for r in runs) + " |"
        sep = "|" + "---|" * (1 + len(runs))
        lines.append(header)
        lines.append(sep)
        for plain, scen in zip(rows_plain, scenarios_ref):
            if scen["device"] != device:
                continue
            md = scenario_row_markdown(scen)
            cells = [fmt_cell(mat[plain][r["id"]]) for r in runs]
            lines.append("| " + md + " | " + " | ".join(cells) + " |")
        lines.append("")

    lines.append("**Warm throughput (mean tok/s, generation phase)**")
    lines.append("")
    emit_device_table("CPU", matrix_warm, "cpu")
    emit_device_table("GPU", matrix_warm, "gpu")
    return "\n".join(lines).rstrip()


def markdown_readme_section(runs: list[dict[str, str]], tables: str) -> str:
    model_list = ", ".join(m.replace("-Q4_K_M", "") for m in BENCH_MODELS)
    lines = [
        "These numbers come from checked-in benchmark outputs. Each **column** is the **latest run** "
        f"for that machine under `reports/`; each **row** is one of **36 scenarios** from `bench.toml` "
        f"({len(BENCH_MODELS)} models × three workloads × CPU/GPU): {model_list}. "
        "Every cell is the rounded measured **tokens/s** for the warm generation phase "
        "(repeat after model load). "
        "**CPU** and **GPU** use separate tables. "
        "An em dash (—) means that machine's latest run did not include that scenario "
        "(for example, older two-model sample runs). "
        "**`llama_cpp_version`** was often unset in older captures—treat cross-host deltas as directional, "
        "not laboratory-precise.",
        "",
        "Scenario labels: **Summarization** asks for a one-sentence summary of a fixed ~600-token article "
        "and caps output at 10 tokens; **Code Generation** asks for a Python `merge_sorted_arrays` "
        "implementation plus asserts and caps output at 512 tokens; **Generic Assistant** asks for practical "
        "planning advice for a 7-day solo hiking trip and caps output at 256 tokens.",
        "",
        "**Latest run per machine**",
        "",
    ]
    for run in runs:
        lines.append(
            f"- [{run['label']}]({run['report']}) — run `{run['run_id']}`"
        )
    lines.extend(["", tables, ""])
    lines.append(
        "_Regenerate these tables and the HTML view with "
        "`python scripts/gen_cross_machine_matrix.py`._"
    )
    return "\n".join(lines)


def patch_readme(content: str) -> None:
    text = README_PATH.read_text(encoding="utf-8")
    if README_BEGIN not in text or README_END not in text:
        raise SystemExit(
            f"README.md missing {README_BEGIN} / {README_END} markers — add them around the cross-machine section"
        )
    before, rest = text.split(README_BEGIN, 1)
    _, after = rest.split(README_END, 1)
    README_PATH.write_text(
        before + README_BEGIN + "\n" + content + "\n" + README_END + after,
        encoding="utf-8",
    )


def write_html(
    path: Path,
    runs: list[dict[str, str]],
    ordered_labels_plain: list[str],
    matrix_warm: dict[str, dict[str, float | None]],
) -> None:
    run_meta = [{"id": r["id"], "label": r["label"], "run_id": r["run_id"]} for r in runs]
    payload = {
        "runs": run_meta,
        "rows": ordered_labels_plain,
        "warm": matrix_warm,
    }
    matrix_json = json.dumps(payload, separators=(",", ":"))

    doc = """<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <title>llmb — cross-machine throughput</title>
  <style>
    :root {{
      font-family: ui-sans-serif, system-ui, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      --bg: #0f1419;
      --fg: #e6edf3;
      --muted: #8b949e;
      --border: #30363d;
    }}
    body {{
      margin: 0;
      background: var(--bg);
      color: var(--fg);
      line-height: 1.45;
      padding: 1.25rem 1.5rem 2rem;
    }}
    h1 {{ font-size: 1.15rem; font-weight: 650; margin: 0 0 0.35rem; }}
    p.lead {{ color: var(--muted); margin: 0 0 1rem; font-size: 0.92rem; max-width: 52rem; }}
    table {{
      border-collapse: collapse;
      font-size: 0.78rem;
      width: max-content;
      max-width: 100%;
    }}
    th, td {{
      border: 1px solid var(--border);
      padding: 0.35rem 0.5rem;
      text-align: right;
    }}
    th:first-child, td:first-child {{
      text-align: left;
      min-width: 14rem;
      max-width: 22rem;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }}
    th {{ background: #161b22; font-weight: 600; }}
    .cell {{
      font-variant-numeric: tabular-nums;
      font-weight: 600;
      color: #0f1419;
    }}
    tr.section td {{
      background: #161b22;
      color: #8b949e;
      font-weight: 600;
      text-align: left;
      font-size: 0.72rem;
      text-transform: uppercase;
      letter-spacing: 0.04em;
      border-top: 1px solid #30363d;
    }}
    footer {{ margin-top: 1.25rem; font-size: 0.78rem; color: var(--muted); }}
  </style>
</head>
<body>
  <h1>Cross-machine throughput</h1>
  <p class="lead">
    Latest <code>results.json</code> per hardware configuration under <code>reports/</code>
    (full <code>bench.toml</code> matrix: six models × three workloads × CPU/GPU = 36 scenarios).
    Rows list all <strong>CPU</strong> scenarios first, then <strong>GPU</strong>.
    Cells are rounded warm mean tokens/s (repeat after model load).
    Em dashes mark scenarios missing from a machine's latest run.
  </p>
  <div id="wrap"></div>
  <footer>Generated by <code>scripts/gen_cross_machine_matrix.py</code>. Re-run after adding benchmark reports.</footer>
  <script type="application/json" id="matrix-data">__MATRIX_JSON__</script>
  <script>
    const DATA = JSON.parse(document.getElementById('matrix-data').textContent);

    function numericValues(mat) {{
      const out = [];
      for (const row of DATA.rows) {{
        for (const run of DATA.runs) {{
          const v = mat[row][run.id];
          if (typeof v === 'number' && !Number.isNaN(v)) out.push(v);
        }}
      }}
      return out;
    }}

    function heatColor(val, vmin, vmax) {{
      if (val == null || vmax <= vmin) return '#2d3748';
      let t = (val - vmin) / (vmax - vmin);
      t = Math.max(0, Math.min(1, t));
      const lo = [45, 35, 52], hi = [43, 96, 56];
      const rgb = lo.map((a, i) => Math.round((a / 100 + (hi[i]/100 - a/100) * t) * 255));
      return '#' + rgb.map(x => x.toString(16).padStart(2, '0')).join('');
    }}

    function render() {{
      const mat = DATA.warm;
      const vals = numericValues(mat);
      if (!vals.length) {{
        document.getElementById('wrap').innerHTML = '<p>No numeric data.</p>';
        return;
      }}
      const vmin = Math.min(...vals);
      const vmax = Math.max(...vals);

      let h = '<table><thead><tr><th>Scenario</th>';
      for (const run of DATA.runs) {{
        const title = run.run_id ? run.label + ' (' + run.run_id + ')' : run.label;
        h += '<th title="' + escapeHtml(title) + '">' + escapeHtml(run.label) + '</th>';
      }}
      h += '</tr></thead><tbody>';
      let prevSection = null;
      for (const row of DATA.rows) {{
        const section = row.endsWith(' · CPU') ? 'CPU' : row.endsWith(' · GPU') ? 'GPU' : '';
        if (section && section !== prevSection) {{
          const colspan = DATA.runs.length + 1;
          h += '<tr class="section"><td colspan="' + colspan + '">' + section + '</td></tr>';
          prevSection = section;
        }}
        const displayRow = (() => {{
          const p = row.split(' · ');
          if (p.length >= 2 && (p[p.length - 1] === 'CPU' || p[p.length - 1] === 'GPU')) {{
            p.pop();
            return p.join(' · ');
          }}
          return row;
        }})();
        h += '<tr><td>' + escapeHtml(displayRow) + '</td>';
        for (const run of DATA.runs) {{
          const v = mat[row][run.id];
          const txt = v == null ? '—' : String(Math.round(v));
          const bg = v == null ? '#21262d' : heatColor(v, vmin, vmax);
          const fg = v == null ? '#8b949e' : '#0f1419';
          h += '<td class="cell" style="background:' + bg + ';color:' + fg + '">' + txt + '</td>';
        }}
        h += '</tr>';
      }}
      h += '</tbody></table>';
      document.getElementById('wrap').innerHTML = h;
    }}

    function escapeHtml(s) {{
      return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
    }}

    render();
  </script>
</body>
</html>
"""
    doc = doc.replace("{{", "{").replace("}}", "}")
    path.write_text(doc.replace("__MATRIX_JSON__", matrix_json), encoding="utf-8")


def build_matrices(
    runs: list[dict[str, str]], scenarios_ref: list[dict]
) -> tuple[list[str], dict[str, dict[str, float | None]]]:
    ordered_plain = [scenario_row_plain(s) for s in scenarios_ref]

    matrix_warm: dict[str, dict[str, float | None]] = {plain: {} for plain in ordered_plain}

    for run in runs:
        scen = load_scenarios(REPO_ROOT / run["json"])
        by_key = {(s["model_name"], s["workload_id"], s["device"]): s for s in scen}
        for s in scenarios_ref:
            plain = scenario_row_plain(s)
            k = (s["model_name"], s["workload_id"], s["device"])
            src = by_key.get(k)
            if not src:
                matrix_warm[plain][run["id"]] = None
                continue
            matrix_warm[plain][run["id"]] = tok_mean(src["warm_metrics"])

    return ordered_plain, matrix_warm


def main() -> None:
    runs = discover_latest_runs(REPORTS_DIR)

    print("Using latest report per configuration:")
    for run in runs:
        print(f"  {run['label']}: {run['json']} (run {run['run_id']})")

    scenarios_ref = canonical_scenarios()
    if len(scenarios_ref) != 36:
        raise SystemExit(f"expected 36 scenarios, got {len(scenarios_ref)}")

    ordered_plain, matrix_warm = build_matrices(runs, scenarios_ref)

    tables = markdown_tables(runs, matrix_warm, ordered_plain, scenarios_ref)
    readme_section = markdown_readme_section(runs, tables)

    snippet_path = REPO_ROOT / "scripts" / "README-cross-machine-snippet.generated.md"
    snippet_path.write_text(readme_section + "\n", encoding="utf-8")
    print(f"wrote {snippet_path.relative_to(REPO_ROOT)}")

    patch_readme(readme_section)
    print(f"updated {README_PATH.relative_to(REPO_ROOT)}")

    out_html = REPO_ROOT / "reports/samples/cross-machine-matrix.html"
    write_html(out_html, runs, ordered_plain, matrix_warm)
    print(f"wrote {out_html.relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
