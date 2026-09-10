# RTX 5090 vs RTX 4080 SUPER

Same-system GPU comparison: **Intel Core Ultra 9 285K**, 32 GB DDR5-6400.

| Workload | 4080S run | 5090 run |
|----------|-----------|----------|
| WoW (CapFrameX) | 2026-05-17 | 2026-05-30 |
| LLM (llama.cpp / llmb) | 20260517T113755 | 20260530T203552 |

## Headline results

| Metric | Result |
|--------|--------|
| WoW median avg FPS (normal) | ~+1% (5090) |
| WoW median 1% low (normal) | ~−10% (5090 worse) |
| WoW max++ median avg FPS | ~+94% (5090) |
| LLM median GPU tok/s | ~+72% (5090) |

> CapFrameX reports **DX12** for all 5090 WoW captures but **DX11** for every 4080S capture — even when comments say "dx12". Treat WoW as same-route/same-settings, not guaranteed same API.

---

## WoW — normal settings

### Average FPS

![WoW average FPS — normal settings](charts/wow-avg-normal.png)

### 1% low FPS

Bottom 1% of frames by FPS (worst sustained dips, not single spikes). At normal settings, 1% lows often favor the 4080S despite similar averages.

![WoW 1% low FPS — normal settings](charts/wow-p1-normal.png)

### 0.1% low FPS

![WoW 0.1% low FPS — normal settings](charts/wow-p01-normal.png)

### 0.01% low FPS

![WoW 0.01% low FPS — normal settings](charts/wow-p001-normal.png)

### 99th percentile FPS

![WoW 99th percentile FPS — normal settings](charts/wow-p99-normal.png)

### Gain %

![WoW gain % — normal settings](charts/wow-gain-normal.png)

---

## WoW — max++ (all sliders maxed)

### Average FPS

![WoW average FPS — max++](charts/wow-avg-maxpp.png)

RE→SoL max++: 5090 **87.6** vs 4080S **38.1** fps (+130%). SoL→RE: **72.6** vs **45.9** (+58%).

### 1% low FPS

![WoW 1% low FPS — max++](charts/wow-p1-maxpp.png)

### Gain %

![WoW gain % — max++](charts/wow-gain-maxpp.png)

---

## LLM — warm GPU tok/s (Q4_K_M, llama.cpp)

### By model

![LLM warm GPU tok/s by model](charts/llm-tps-by-model.png)

### Gain % by model

![LLM 5090 gain % by model](charts/llm-gain-by-model.png)

Larger models benefit more: Qwen3-8B ~2×, Qwen3-0.6B ~1.3×.

### Per model × workload

![LLM warm GPU tok/s per workload](charts/llm-by-workload.png)

---

## Regenerate

```bash
python analyze.py           # CapFrameX captures + llmb results.csv → comparison_data.json
python render_charts.py     # comparison_data.json → charts/*.png
python generate_report.py   # optional interactive report.html (Chart.js)
python push_to_labs.py      # push this folder to github.com/folkol/labs
```

**Data sources** (local paths on the benchmark machine):

- CapFrameX: `%USERPROFILE%\Documents\CapFrameX\Captures\`
- LLM: `llm-inference-benchmark/reports/windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_*`

Interactive version: open [`report.html`](report.html) in a browser.
