# Local model eval

Speed and quality of local models against a qwen3.8-27b baseline. `uv run e2e.py` writes one `REPORT.md`.

## Findings

2026-11-22, qwen3.8-27b, Ornith-1.5-35B-A3B, and Inkling-2-Mini. The full table is in [REPORT.md](REPORT.md).

| | qwen3.8-27b | ornith-1.5-35b-a3b | inkling-2-mini |
|---|---|---|---|
| TTFT | 2212 ms | 2156 ms | 219 ms |
| tok/s | 92.5 | 246.4 (2.7× faster) | 30.9 (3.0× slower) |
| turn, thinking off | 2.3 s | 2.2 s | 2.9 s |
| turn, medium | 2.6 s | 2.4 s | 2.5 s |
| json, constraints, math, coding, reasoning, fix_it, triage | 100 | 100 | 100 |
| summarize | 67* | 67* | 83* |
| rewrite_tone | 67* | 100* | 67* |
| ifeval | 85† | 85† | 70† |
| gsm8k | 95† | 95† | 85† |
| humaneval | 100† | 100† | 95† |

TTFT is milliseconds to the first token. tok/s is decode speed on a fixed 512-token generation. Both are measured with thinking off. The two turn rows are seconds to finish a short edit, thinking off and again at medium effort. A lower turn time is faster.

An unmarked 100 means every exact check passed. A star means qwen3.8 judged the answer on three 0/1/2 criteria: 100 is 6/6, 83 is 5/6, and 67 is 4/6. A dagger is percent correct on 20 samples with thinking off. IFEval uses the strict prompt score.

Ornith matched qwen on every exact-check task and on the three 20-sample tasks, and it was the only one of the three to drop the sarcastic $50, so its rewrite is 6/6. Its summary is the same 4/6 as qwen. It decodes at 2.7× qwen's rate. Time to first token stays about 2.2 s for both, because both run through Ollama.

Inkling also matched the exact-check tasks, and its summary is the high one at 5/6. It kept the $50, and it trails the 20-sample tasks: 70, 85, and 95. It reaches a first token in about 0.2 s and decodes at about a third of qwen's rate. The short edit takes about the same time for all three.

## Machine

The report dated 2026-11-22 was measured on this PC:

- Windows 11 26H2 (build 10.0.26300)
- Intel Core Ultra 9 285K, 24 cores
- 32 GB Corsair DDR5-4800 (2×16 GB)
- NVIDIA GeForce RTX 5090, 32 GB VRAM
- Ollama 0.40.16 for every model except Inkling, which runs on llama-server
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
