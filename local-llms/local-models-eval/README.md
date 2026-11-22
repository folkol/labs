# Local model eval

Speed and quality of local models against a qwen3.8-27b baseline. `uv run e2e.py` writes one `REPORT.md`.

## Findings

2026-11-22, qwen3.8-27b, Ornith-1.5-35B-A3B, and Inkling-2-Mini. The full table is in [REPORT.md](REPORT.md).

| | qwen3.8-27b | ornith-1.5-35b-a3b | inkling-2-mini |
|---|---|---|---|
| TTFT | 1446 ms | 156 ms | 235 ms |
| tok/s | 66.3 | 201.6 (3.0× faster) | 34.3 (1.9× slower) |
| turn, thinking off | 0.3 s | 0.2 s | 2.2 s |
| turn, medium | 1.0 s | 0.3 s | 2.1 s |
| json, constraints, math, coding, reasoning, fix_it, triage | 100 | 100 | 100 |
| summarize | 67* | 83* | 83* |
| rewrite_tone | 100* | 100* | 100* |
| ifeval | 85† | 80† | 70† |
| gsm8k | 95† | 90† | 85† |
| humaneval | 100† | 100† | 95† |

TTFT is milliseconds to the first token. tok/s is decode speed on a fixed 512-token generation. Both are measured with thinking off. The two turn rows are seconds to finish a short edit, thinking off and again at medium effort. A lower turn time is faster.

An unmarked 100 means every exact check passed. A star means qwen3.8 judged the answer on three 0/1/2 criteria: 100 is 6/6, 83 is 5/6, and 67 is 4/6. A dagger is percent correct on 20 samples with thinking off. IFEval uses the strict prompt score.

This run served every candidate with llama-server. Ornith matched qwen on the exact-check tasks and on the rewrite (6/6), and scored higher on the summary (5/6 vs 4/6). It decodes at 3.0× qwen's rate, answers in 156 ms, and finishes the short edit in 0.2–0.3 s. On the 20-sample tasks it is close: 80, 90, and 100 against qwen's 85, 95, and 100.

Inkling matched the exact-check tasks and the rewrite, and tied Ornith on the summary. It trails the 20-sample tasks at 70, 85, and 95. Its first token is 235 ms, in the same range as Ornith, and it decodes at 34 tok/s, so the short edit takes about 2 s.

Inkling 2 Mini is a 26B looped mixture-of-experts model (about 3B active per token) from Thinking Machines Lab, released 2026-11-20. The weights are `thinkingmachines/Inkling-2-Mini`. This eval runs ggml-org's `Inkling-2-Mini-IQ4_NL.gguf`: Pedro Cuenca's conversion, with the NVFP4 experts in layers 0–19 at 4-bit and the other matrices at Q8_0.

### Linux, same PC

The same three models on the same PC booted into Ubuntu 25.10, 2026-11-22. All candidates ran on llama-server with the same GGUF files, and the judge ran on llama-server too. The full table, with gemma-4-26b-a4b and tiel-coder-35b-a3b, is in [REPORT-linux.md](REPORT-linux.md).

| | qwen3.8-27b | ornith-1.5-35b-a3b | inkling-2-mini |
|---|---|---|---|
| TTFT | 102 ms | 62 ms | 93 ms |
| tok/s | 79.8 | 288.6 (3.6× faster) | 95.1 (1.2× faster) |
| turn, thinking off | 0.2 s | 0.1 s | 0.6 s |
| turn, medium | 0.8 s | 0.2 s | 1.1 s |
| json, constraints, math, coding, reasoning, fix_it, triage | 100 | 100 | 100 |
| summarize | 83* | 67* | 83* |
| rewrite_tone | 67* | 100* | 100* |
| ifeval | 90† | 80† | 70† |
| gsm8k | 95† | 90† | 90† |
| humaneval | 100† | 100† | 100† |

Linux decodes faster for every model: qwen 80 vs 66 tok/s, Ornith 289 vs 202, Inkling 95 vs 34. Inkling gains the most, from 1.9× slower than qwen on Windows to 1.2× faster on Linux. First-token times are also lower, and the short edit is faster across the board.

The quality scores are not comparable one-to-one. The 20-sample tasks run once per sample, and the judged tasks were scored by a separate judge instance, so qwen's own summary moved from 67 to 83 and its rewrite from 100 to 67. The exact-check tasks are identical on both systems. Read the judged and 20-sample rows as noise of a point or two per sample, not as an OS effect.

## Machine

Both reports dated 2026-11-22 were measured on this PC, dual-booted:

- Windows 11 26H2 (build 10.0.26300) for `REPORT.md`
- Ubuntu 25.10, kernel 6.17, NVIDIA driver 595.71 for `REPORT-linux.md`
- Intel Core Ultra 9 285K, 24 cores
- 32 GB Corsair DDR5-4800 (2×16 GB)
- NVIDIA GeForce RTX 5090, 32 GB VRAM
- llama-server for every candidate; on Windows the qwen3.8 judge stays on Ollama 0.40.16, on Linux it runs on llama-server
- Context 16384, 12 threads, one model loaded at a time

## Reading the report

qwen3.8-27b is the baseline column. "vs baseline" is a speed ratio against it. Lower turn time is faster.

**Speed**

- **TTFT** is milliseconds to the first token, thinking off.
- **tok/s** is decode speed on a fixed 512-token generation, thinking off. Thinking does not change this rate.
- **Turn no-think** and **turn medium** are seconds to finish a short code edit, from send until the model stops. Medium is thinking on. A `*` on a turn time means at least one of the 3 runs hit the 8192-token cap and was left out of the mean.

**Quality**

Scores are 0–100. Local tasks are the best of 3 runs, with thinking on at medium effort. A row is missing when that run did not score it.

| Mark | Where | Meaning |
|---|---|---|
| none | quality cell | Deterministic. Pass rate of exact checks, or of a hidden unit test that actually runs. |
| `*` | quality cell | LLM-judged by qwen3.8 on a 0/1/2 rubric, scaled to 0–100. The judge also scores its own baseline answers. |
| `*` | turn time | A run hit the token cap. Different from the quality star. |
| `†` | ifeval, gsm8k, humaneval | Percent correct on 20 samples, thinking off, one try each. A thinking trace is graded as a wrong answer. |
| `!` | any timed trial | Paging VRAM: after the first 3 seconds, most busy seconds were under 40% GPU and over 4 GB/s on the bus. Treat that number as a model that did not fit. |
| `—` | any cell | Not run, or the turn never finished under the cap. |

**Notes** name the best candidate on each task. The baseline is left out of that list, so a row of 100s still names every candidate when qwen also scored 100. The recommendation paragraph and the minimax note are fixed prose and can lag the tables. Read the tables.

**VRAM**, when present, lists each model and trial that was paging. A trial of only a couple of seconds is too short to judge.

## Tasks

| Task | What a high score means |
|---|---|
| `json_extract` | Invoice JSON: vendor Bluepeak, due 2027-03-31, amount 4132.50 |
| `constraints` | Exactly 4 short sentences, facts split, banned words absent |
| `math_word` | Answer line is 32 minutes |
| `coding` | `fold_ledger` passes 5 hidden cases (voids, comments, currency) |
| `reasoning` | The unique seating is Dev in the middle |
| `fix_it` | The fix handles a 3rd occurrence, not just a cleanup |
| `summarize` | Two sentences: keeps 34%, 9%, three corridors; drops the rejected curfew and the pre-existing 2% |
| `rewrite_tone` | Keeps the order facts, drops the sarcastic $50 and the bike courier, polite tone |
| `triage` | Picks the passage-supported answer and does not adopt the invented one |
| `ifeval` | Follows a concrete format instruction |
| `gsm8k` | Short grade-school word problem |
| `humaneval` | Small Python function passes its unit tests |

## Run

```
uv run e2e.py
```

Phase 1 is speed and deterministic tasks. Phase 2 is the qwen3.8 judge on the three rubric tasks. Phase 3 is IFEval, GSM8K, and HumanEval, 20 samples each, folded into the same `results.json`. `REPORT.md` is written after that. Re-runs skip finished downloads.

Every candidate is served by llama-server, including qwen, whose GGUF is the large blob already in Ollama's store. The judge stays on Ollama. One run writes `results.json` and `REPORT.md`; nothing is pasted in by hand.

## Linux

Boot the Linux install and run this from the same checkout:

```
uv run e2e-linux.py
```

Every candidate, Inkling included, is served by llama-server with the same GGUF files as on Windows. The judge runs on llama-server too, since the Linux Ollama is too old for qwen3.8. Context stays 16384.

The script writes `results-linux.json`, `REPORT-linux.md` and `external_logs_linux/`. The Windows measurements stay in `results.json` and `REPORT.md`. Setup notes and pitfalls are in [README.linux.md](README.linux.md).

`nvidia-smi` has to see the 5090. The qwen GGUF is the blob on the Windows `E:` drive, so that disk has to be mounted; the script mounts it read-only through `udisksctl`.
