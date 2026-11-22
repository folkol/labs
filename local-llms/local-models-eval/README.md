# Local model eval (RTX 5090, 32 GB)

Evaluates GGUF models on **speed** (TTFT, tokens/s) and **quality** (7 tasks).

## Quick start (end-to-end)

```
uv run e2e.py
```
(uses `uv` — Python deps are auto-installed from the inline script metadata)

Does everything: installs pip deps, downloads the latest llama.cpp
Windows CUDA build to `.bin/`, downloads all GGUFs from Hugging Face into
`models/<name>/` (~90 GB total), resolves `gguf` paths in `config.yaml`,
runs speed + deterministic tasks for every model (`--no-judge`), then starts
the judge (baseline qwen3.8 by default, judge in VRAM alone) and scores the
two rubric tasks (`--judge-only`). Results: `results.json` + printed table.
Idempotent — re-runs skip finished downloads.

## Manual setup (if you prefer)

1. llama.cpp with CUDA, `llama-server` on PATH (or pass `--llama <path>`):
   https://github.com/ggml-org/llama.cpp — build with `-DGGML_CUDA=ON`
2. Download GGUFs (examples):
   ```
   hf download unsloth/Qwen3.8-27B-GGUF --include "*Q5_K_M*" --local-dir models
   hf download unsloth/MiniMax-H3-GGUF --include "*Q4_K_M*" --local-dir models
   hf download ornith-ai/Ornith-1.5-35B-A3B-GGUF --include "*Q4_K_M*" --local-dir models
   hf download unsloth/gemma-4-26B-A4B-it-GGUF --include "*Q6_K*" --local-dir models
   ```
   Then point each entry in `config.yaml` at the actual file.
3. `pip install requests pyyaml`

## Judge

Two tasks (`summarize`, `rewrite_tone`) are scored by an LLM judge — any
OpenAI-compatible endpoint in `config.yaml → judge` (e.g. your qwen3.8 on a
second port, or an API). For fairness, the judge should **not** be one of the
candidates. Run without a judge: `--no-judge` (those two tasks are skipped).

## Run

```
python run_eval.py                                # all models in config
python run_eval.py --models minimax-h3 gemma-4-26b-a4b
python run_eval.py --no-judge --runs 5
python test_tasks.py                              # checker smoke tests
```

One model at a time; a fresh `llama-server` (ctx 16384) per model. 3 runs per
task. tok/s is one decode-rate probe: thinking off, fixed 512 tokens. Turn time
is seconds until a short edit finishes, measured with thinking off and again
at medium effort. Quality tasks run at medium effort with thinking on, and the
score is the best run. Server logs in `logs/`.

## Tasks & rubric

Each task scores 0–100; overall quality = mean of task scores.

| Task | Type | Scoring |
|---|---|---|
| `json_extract` | invoice thread: credit, replaced due date, distractor vendor | 4 exact checks (parses, vendor, ISO date, net amount) |
| `constraints` | 4-sentence summary, short lines, facts split, banned words | 8 rule checks (count, length, split numbers, bans, diesel first, periods) |
| `math_word` | two speeds plus a reprint pass (answer: 32 min) | answer line present + exact value |
| `coding` | `fold_ledger` with voids, comments, and currency conversion | hidden 5-case unit test actually executed |
| `reasoning` | 5-seat ordering puzzle with one valid arrangement (answer: Dev) | answer present + correct name |
| `summarize` | 2-sentence summary that must drop a rejected proposal and a decoy statistic | judge ×3: accuracy, coverage, selection (0/1/2 each) |
| `rewrite_tone` | facts buried in insults; a sarcastic $50 must be dropped | judge ×3: facts preserved, decoy omitted, tone (0/1/2 each) |

## Reading the results

- **tok/s**: MoE A3B/A4B models should beat the 27B dense by 2–4× — that's the 5090 doing its job.
- **TTFT**: first-token latency; matters for interactive use.
- A model within ~5 pts on quality at 2–3× the speed usually wins for local use.

## Misc

```
nvidia-smi dmon -i 0 -s putcm
llama-server -hf ggml-org/Inkling-2-Mini-GGUF:IQ4_NL -c 65536 -np 1 -ngl 99 --reasoning-effort medium
```
