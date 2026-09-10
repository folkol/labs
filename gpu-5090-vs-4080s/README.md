# RTX 5090 vs RTX 4080 SUPER

Same-system GPU comparison: **Intel Core Ultra 9 285K**, 32 GB DDR5-6400.

| Workload | 4080S run | 5090 run |
|----------|-----------|----------|
| WoW (CapFrameX) | 2026-05-17 | 2026-05-30 |
| LLM (llama.cpp / llmb) | 20260517T113755 | 20260530T203552 |

## View the report

Open **`report.html`** in a browser (requires network for Chart.js CDN).

Charts included:

- WoW average FPS (normal + max++)
- WoW 1% / 0.1% / 0.01% low FPS
- WoW 99th percentile FPS
- WoW gain % by scenario
- LLM warm GPU tok/s (by model and per workload)

## Regenerate

```bash
python analyze.py          # reads CapFrameX captures + llmb results.csv
python generate_report.py  # writes report.html from comparison_data.json
```

**Data sources** (local paths on the benchmark machine):

- CapFrameX: `%USERPROFILE%\Documents\CapFrameX\Captures\`
- LLM: `llm-inference-benchmark/reports/windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_*`

## Caveats

CapFrameX reports **DX12** for all 5090 WoW captures but **DX11** for every 4080S capture — even when comments say "dx12". Treat WoW as same-route/same-settings, not guaranteed same API.

## Headline results

| Metric | Result |
|--------|--------|
| WoW median avg FPS (normal) | ~+1% (5090) |
| WoW median 1% low (normal) | ~−10% (5090 worse) |
| WoW max++ median avg FPS | ~+94% (5090) |
| LLM median GPU tok/s | ~+72% (5090) |
