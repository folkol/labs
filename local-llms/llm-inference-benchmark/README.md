# llmb – Cross-Platform LLM Inference Benchmark

A single compiled CLI that measures LLM inference performance across machines and produces comparable **tokens/second** reports (plus load time and TTFT).

## Cross-machine throughput

<!-- BEGIN cross-machine-matrix -->
These numbers come from checked-in benchmark outputs. Each **column** is the **latest run** for that machine under `reports/`; each **row** is one of **36 scenarios** from `bench.toml` (6 models × three workloads × CPU/GPU): Qwen3-0.6B, SmolLM3-3B, Qwen3-4B, Gemma-3-4B-IT, Phi-4-mini-instruct, Qwen3-8B. Every cell is the rounded measured **tokens/s** for the warm generation phase (repeat after model load). **CPU** and **GPU** use separate tables. An em dash (—) means that machine's latest run did not include that scenario (for example, older two-model sample runs). **`llama_cpp_version`** was often unset in older captures—treat cross-host deltas as directional, not laboratory-precise.

Scenario labels: **Summarization** asks for a one-sentence summary of a fixed ~600-token article and caps output at 10 tokens; **Code Generation** asks for a Python `merge_sorted_arrays` implementation plus asserts and caps output at 512 tokens; **Generic Assistant** asks for practical planning advice for a 7-day solo hiking trip and caps output at 256 tokens.

**Latest run per machine**

- [Windows · Ultra 9 285K · RTX 4080 SUPER](reports/windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_4080_SUPER__20260517T113755/report.html) — run `20260517T113755`
- [Linux · Ultra 9 285K · RTX 4080 SUPER](reports/linux-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_4080_SUPER__20260517T004235/report.html) — run `20260517T004235`
- [Windows · Ultra 9 285K · RTX 5090](reports/windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_5090__20260601T180932/report.html) — run `20260601T180932`
- [Windows · Ryzen 9 9950X3D · RTX 5090](reports/samples/windows-x86_64__CPU-AMD_Ryzen_9_9950X3D_16_Core_Processor__GPU-NVIDIA_GeForce_RTX_5090__20260517T122142/report.html) — run `20260517T122142`
- [macOS · Apple M4 · Apple Silicon GPU](reports/macos-aarch64__CPU-Apple_M4__GPU-Apple_Silicon_GPU__20260516T232836/report.html) — run `20260516T232836`

**Warm throughput (mean tok/s, generation phase)**

**CPU**

| Scenario | [Windows · Ultra 9 285K · RTX 4080 SUPER](reports/windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_4080_SUPER__20260517T113755/report.html) | [Linux · Ultra 9 285K · RTX 4080 SUPER](reports/linux-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_4080_SUPER__20260517T004235/report.html) | [Windows · Ultra 9 285K · RTX 5090](reports/windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_5090__20260601T180932/report.html) | [Windows · Ryzen 9 9950X3D · RTX 5090](reports/samples/windows-x86_64__CPU-AMD_Ryzen_9_9950X3D_16_Core_Processor__GPU-NVIDIA_GeForce_RTX_5090__20260517T122142/report.html) | [macOS · Apple M4 · Apple Silicon GPU](reports/macos-aarch64__CPU-Apple_M4__GPU-Apple_Silicon_GPU__20260516T232836/report.html) |
|---|---|---|---|---|---|
| Qwen3-0.6B · Summarization | 156 | 86 | 112 | 106 | 115 |
| Qwen3-0.6B · Code Generation | 139 | 79 | 130 | 105 | 95 |
| Qwen3-0.6B · Generic Assistant | 139 | 82 | 129 | 111 | 104 |
| SmolLM3-3B · Summarization | 45 | — | 44 | — | — |
| SmolLM3-3B · Code Generation | 40 | — | 41 | — | — |
| SmolLM3-3B · Generic Assistant | 40 | — | 41 | — | — |
| Qwen3-4B · Summarization | 35 | 24 | 35 | 24 | 31 |
| Qwen3-4B · Code Generation | 31 | 25 | 31 | 21 | 28 |
| Qwen3-4B · Generic Assistant | 31 | 26 | 32 | 22 | 27 |
| Gemma-3-4B-IT · Summarization | 35 | — | 35 | — | — |
| Gemma-3-4B-IT · Code Generation | 31 | — | 31 | — | — |
| Gemma-3-4B-IT · Generic Assistant | 31 | — | 31 | — | — |
| Phi-4-mini-instruct · Summarization | 36 | — | 37 | — | — |
| Phi-4-mini-instruct · Code Generation | 33 | — | 33 | — | — |
| Phi-4-mini-instruct · Generic Assistant | 33 | — | 33 | — | — |
| Qwen3-8B · Summarization | 20 | — | 20 | — | — |
| Qwen3-8B · Code Generation | 18 | — | 18 | — | — |
| Qwen3-8B · Generic Assistant | 18 | — | 18 | — | — |

**GPU**

| Scenario | [Windows · Ultra 9 285K · RTX 4080 SUPER](reports/windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_4080_SUPER__20260517T113755/report.html) | [Linux · Ultra 9 285K · RTX 4080 SUPER](reports/linux-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_4080_SUPER__20260517T004235/report.html) | [Windows · Ultra 9 285K · RTX 5090](reports/windows-x86_64__CPU-Intel_Core_Ultra_9_285K__GPU-NVIDIA_GeForce_RTX_5090__20260601T180932/report.html) | [Windows · Ryzen 9 9950X3D · RTX 5090](reports/samples/windows-x86_64__CPU-AMD_Ryzen_9_9950X3D_16_Core_Processor__GPU-NVIDIA_GeForce_RTX_5090__20260517T122142/report.html) | [macOS · Apple M4 · Apple Silicon GPU](reports/macos-aarch64__CPU-Apple_M4__GPU-Apple_Silicon_GPU__20260516T232836/report.html) |
|---|---|---|---|---|---|
| Qwen3-0.6B · Summarization | 540 | 628 | 713 | 768 | 135 |
| Qwen3-0.6B · Code Generation | 500 | 611 | 656 | 683 | 161 |
| Qwen3-0.6B · Generic Assistant | 507 | 621 | 657 | 684 | 89 |
| SmolLM3-3B · Summarization | 239 | — | 406 | — | — |
| SmolLM3-3B · Code Generation | 223 | — | 395 | — | — |
| SmolLM3-3B · Generic Assistant | 223 | — | 380 | — | — |
| Qwen3-4B · Summarization | 189 | 220 | 321 | 338 | 39 |
| Qwen3-4B · Code Generation | 175 | 199 | 303 | 308 | 36 |
| Qwen3-4B · Generic Assistant | 176 | 201 | 304 | 310 | 36 |
| Gemma-3-4B-IT · Summarization | 167 | — | 242 | — | — |
| Gemma-3-4B-IT · Code Generation | 164 | — | 264 | — | — |
| Gemma-3-4B-IT · Generic Assistant | 164 | — | 265 | — | — |
| Phi-4-mini-instruct · Summarization | 206 | — | 385 | — | — |
| Phi-4-mini-instruct · Code Generation | 188 | — | 352 | — | — |
| Phi-4-mini-instruct · Generic Assistant | 190 | — | 350 | — | — |
| Qwen3-8B · Summarization | 122 | — | 239 | — | — |
| Qwen3-8B · Code Generation | 110 | — | 216 | — | — |
| Qwen3-8B · Generic Assistant | 110 | — | 222 | — | — |

_Regenerate these tables and the HTML view with `python scripts/gen_cross_machine_matrix.py`._
<!-- END cross-machine-matrix -->

## Latest local run (Windows · Ultra 9 285K · RTX 4080 SUPER)

This run (`20260517T113755`) uses the expanded six-model matrix in `bench.toml`. Values are rounded measured generation **tokens/s**; **warm** is the repeat after load and **cold** is the first completion in that scenario.

| Model / workload | CPU warm | CPU cold | GPU warm | GPU cold |
|---|---:|---:|---:|---:|
| Qwen3-0.6B · Summarization | 156 | 157 | 540 | 341 |
| Qwen3-0.6B · Code Generation | 139 | 138 | 500 | 499 |
| Qwen3-0.6B · Generic Assistant | 139 | 141 | 507 | 501 |
| SmolLM3-3B · Summarization | 45 | 45 | 239 | 212 |
| SmolLM3-3B · Code Generation | 40 | 40 | 223 | 225 |
| SmolLM3-3B · Generic Assistant | 40 | 41 | 223 | 222 |
| Qwen3-4B · Summarization | 35 | 35 | 189 | 169 |
| Qwen3-4B · Code Generation | 31 | 31 | 175 | 175 |
| Qwen3-4B · Generic Assistant | 31 | 31 | 176 | 177 |
| Gemma-3-4B-IT · Summarization | 35 | 34 | 167 | 160 |
| Gemma-3-4B-IT · Code Generation | 31 | 31 | 164 | 164 |
| Gemma-3-4B-IT · Generic Assistant | 31 | 31 | 164 | 164 |
| Phi-4-mini-instruct · Summarization | 36 | 36 | 206 | 188 |
| Phi-4-mini-instruct · Code Generation | 33 | 33 | 188 | 189 |
| Phi-4-mini-instruct · Generic Assistant | 33 | 33 | 190 | 190 |
| Qwen3-8B · Summarization | 20 | 20 | 122 | 115 |
| Qwen3-8B · Code Generation | 18 | 18 | 110 | 110 |
| Qwen3-8B · Generic Assistant | 18 | 18 | 110 | 110 |

## What it measures

| Dimension | Details |
|-----------|---------|
| **Models** | Any GGUF model; curated samples above use **Qwen3** (0.6B / 4B); defaults in `bench.toml` may include additional families |
| **Weight sizes** | Configurable; start with Q4_K_M quantized models that run on CPU |
| **Workloads** | Summarization, code generation, generic assistant (fixed prompts for reproducibility) |
| **Devices** | `cpu`, `gpu`, `auto` via llama.cpp flags |
| **Run modes** | First request after server load (“cold”), then **warm_runs** sequential requests on the same loaded `llama-server` (no composite score—raw metrics only) |

## Metrics collected

- **Tokens/second** — measured generation throughput for the **cold** first completion and configured **warm** repeats (current default: one warm repeat)
- **Load time** — model ready (from server start through health / first use, as recorded per scenario)
- **Time to first token (TTFT)** on the cold path
- **Memory** — where available from llama.cpp output

Concurrent batch benchmarking is optional via `batch_sizes` in config; default configs leave it empty.

1. **llama.cpp** — `llama-server` must be on `PATH` or installed via `llmb setup` (default benchmarks drive the HTTP API).
   - Windows: download prebuilt ZIP from [llama.cpp releases](https://github.com/ggerganov/llama.cpp/releases) and add the folder to `PATH`.
   - macOS/Linux: `brew install llama.cpp` or build from source.
2. **Disk space** — models range from ~400 MB (0.5B Q4) to ~5 GB (8B Q4). The default set is ~7 GB total.
3. **RAM** — 0.5–1.5B models need ~1 GB; 3B needs ~2.5 GB; 7–8B needs ~5–6 GB.

## Quick start

```sh
# 1. Download or build the llmb binary (or cargo build --release)
cargo build --release
cp target/release/llmb ~/.local/bin/   # Linux/macOS
# On Windows add target\release to PATH

# 2. Create a config in your working directory
llmb init

# 3. Download models (large download, ~7 GB for defaults)
llmb models fetch

# 4. Run the benchmark matrix from bench.toml (default config uses cpu + gpu when available)
llmb bench

# 5. Open the report
llmb report
```

The default output folder is `./reports/<os-arch>__CPU-…__GPU-…__<run-id>/` (detected hardware plus a UTC `run_id`). Use `--out DIR` for a stable path (`llmb bench --out reports/latest` restores the previous layout).

Without arguments, `llmb report` opens the newest `report.html` anywhere under `./reports/` (recursive, so it includes `./reports/samples/...`).

### Multiple machines (Windows, Linux, other PCs)

Every run defaults to `./reports/<os-arch>__CPU-…__GPU-…__<run-id>/`, so hosts usually get **distinct directory names**: e.g. `linux-x86_64__…` vs `windows-x86_64__…`, and different GPUs or CPUs show up in the slug. Repeated runs on the same box differ by `run-id` (UTC timestamp).

Runs under `./reports/` are **local-only** unless you publish them on purpose. To keep curated reports in git (benchmarks from this machine under Linux, or tarballs copied from another computer), drop each finished run folder under **`reports/samples/`** — one subdirectory per run. That tree is tracked; anything else directly under `./reports/` stays ignored.

## GPU benchmarking

Edit `bench.toml` and change `devices`:

```toml
devices = ["cpu", "gpu"]
```

For NVIDIA (CUDA), use a CUDA-enabled llama.cpp binary. For Apple Silicon, use the Metal-enabled build. The tool passes `--n-gpu-layers -1` for GPU (all layers) and `--n-gpu-layers 0` for CPU.

## Commands

```
llmb init                          # write bench.toml to current directory
llmb models list                   # show models and cache status
llmb models fetch                  # download missing models
llmb bench                         # run with bench.toml defaults
llmb bench --devices cpu,gpu       # override devices
llmb bench --models Qwen2.5-0.5B   # run only matching models
llmb bench --runs 5                # 5 warm repetitions instead of 3
llmb bench --out reports/my-run    # fixed directory (overwrite same path each run)
llmb report                        # newest report under ./reports/
llmb report reports/my-run         # open one run explicitly
```

## Output files

Each run writes three files to the output directory:

| File | Contents |
|------|---------|
| `results.json` | Canonical output: hardware info, `schema_version`, every scenario’s cold/warm metrics and raw samples |
| `results.csv` | Flat rows keyed by model × workload × device — primarily **tokens/sec**, load, TTFT |
| `report.html` | Chart.js bar charts: warm tok/s, cold tok/s, load time, TTFT |

## Comparing machines

Run `llmb bench` on each machine and collect `results.csv` (or feed several `results.json` files into **`llmb compare`**). Workloads and token budgets are fixed in the binary, so **tokens/second** columns are directly comparable given the same model file and similar llama.cpp builds. Per-scenario matrices at the top of this README mirror those checked-in runs; **`report.html`** for each machine adds TTFT and load-time plots.

## Config reference

```toml
# bench.toml
devices      = ["cpu", "gpu"]
warm_runs    = 1
batch_sizes  = []            # optional: e.g. [1, 4, 8] for batch experiments
max_tokens   = 256
cpu_threads  = 0
gpu_layers   = -1
mixed_gpu_layers = 16

[[models]]
name         = "Qwen3-0.6B-Q4_K_M"
filename     = "Qwen_Qwen3-0.6B-Q4_K_M.gguf"
url          = "https://huggingface.co/..."
params       = "0.6B"
quantization = "Q4_K_M"
context_length = 1024
sha256       = "optional hex digest for verification"
```

## Adding custom models

Any GGUF model from Hugging Face (or a local path as a `file://` URL) can be added to the `[[models]]` section. The benchmark will skip models whose files are not present in the cache and remind you to run `llmb models fetch`.

## Model cache location

| OS | Cache path |
|----|-----------|
| Windows | `%LOCALAPPDATA%\llm-benchmark` |
| macOS | `~/Library/Caches/llm-benchmark` |
| Linux | `~/.cache/llm-benchmark` |

## Building from source on each platform

```sh
# All platforms (requires Rust 1.80+)
git clone <repo>
cd llm-inference-benchmark
cargo build --release

# The binary is at:
#   target/release/llmb          (Linux/macOS)
#   target\release\llmb.exe      (Windows)
```

On Windows without Visual Studio, use the GNU toolchain:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
# Add MinGW64 bin directory to PATH, then:
cargo build --release
```

## Reproducibility notes

- All workload prompts are hardcoded in the binary (not read from disk at runtime) so they cannot drift between runs.
- The random seed is fixed to 42 for all llama.cpp calls.
- Hardware information (CPU, RAM, GPU, OS version) is recorded in every `results.json`.
- llama.cpp version is included when detectable.
