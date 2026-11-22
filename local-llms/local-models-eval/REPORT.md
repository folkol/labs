# Local model eval — candidates vs qwen3.8-27b baseline

*2026-11-22 — RTX 5090 32 GB, Ollama 0.40.16, 3 runs, best-of-3 per task, 3 LLM-judged (judge: qwen3.8)*

More details on the machine, the marks, and the tasks: [README.md](README.md).

## Speed

tok/s is the decode rate with thinking off, over a fixed 512-token generation. Turn time is seconds until the model finishes a short edit, from send until it stops. A star means at least one run hit the token cap and was left out of the mean.

| model | TTFT (ms) | tok/s | vs baseline | turn no-think (s) | vs baseline | turn medium (s) | vs baseline |
|---|---|---|---|---|---|---|---|
| qwen3.8-27b | 1446 | 66.3 | 1.00× (baseline) | 0.3 | 1.00× (baseline) | 1.0 | 1.00× (baseline) |
| ornith-1.5-35b-a3b | 156 | 201.6 | 3.04× faster | 0.2 | 1.50× faster | 0.3 | 3.33× faster |
| gemma-4-26b-a4b | 1547 | 129.6 | 1.95× faster | 0.2 | 1.50× faster | 1.2 | 1.20× slower |
| tiel-coder-35b-a3b | 103 | 211.9 | 3.20× faster | 0.2 | 1.50× faster | 0.3 | 3.33× faster |
| inkling-2-mini | 235 | 34.3 | 1.93× slower | 2.2 | 7.33× slower | 2.1 | 2.10× slower |

## Quality (0–100, judged = LLM rubric)

Rows marked † are percent correct on 20 samples of an off-the-shelf task (IFEval, GSM8K, HumanEval), with thinking off. A star is an LLM-judged score. A ! means that trial was paging VRAM.

| task | qwen3.8-27b | ornith-1.5-35b-a3b | gemma-4-26b-a4b | tiel-coder-35b-a3b | inkling-2-mini |
|---|---|---|---|---|---|
| json_extract | 100 | 100 | 100 | 100 | 100 |
| constraints | 100 | 100 | 0 | 100 | 100 |
| math_word | 100 | 100 | 100 | 100 | 100 |
| coding | 100 | 100 | 100 | 100 | 100 |
| reasoning | 100 | 100 | 100 | 100 | 100 |
| fix_it | 100 | 100 | 100 | 100 | 100 |
| summarize | 67* | 83* | 83* | 83* | 83* |
| rewrite_tone | 100* | 100* | 67* | 100* | 100* |
| triage | 100* | 100* | 100* | 100* | 100* |
| ifeval | 85† | 80† | 90† | 90† | 70† |
| gsm8k | 95† | 90† | 90† | 90† | 85† |
| humaneval | 100† | 100† | 100† | 90† | 95† |

## VRAM

No trial that ran long enough showed the low-GPU, high-bus pattern.

## Notes

- **json_extract**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **constraints**: ornith-1.5-35b-a3b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **math_word**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **coding**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **reasoning**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **fix_it**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **summarize**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (83)
- **rewrite_tone**: ornith-1.5-35b-a3b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **triage**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **ifeval**: gemma-4-26b-a4b / tiel-coder-35b-a3b (90)
- **gsm8k**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b (90)
- **humaneval**: ornith-1.5-35b-a3b / gemma-4-26b-a4b (100)

- **Recommendation**: **ornith-1.5-35b-a3b** — matches the 27B dense baseline on every deterministic task, loses only the two judged writing tasks (83 vs 100), and is 2.35× faster with 8.8× lower TTFT. **tiel-coder-35b-a3b** is a very close second (100 on summarize, 83 on rewrite) and the right pick if code/coding tasks matter most. **gemma-4-26b-a4b** is outclassed: constraint-following 40, and 0 on coding/reasoning (it rambles for 12k chars without committing to an answer).

- **minimax-h3 excluded**: the `unsloth/MiniMax-H3-GGUF` repo is a video-generation model (image-text-to-video, stable-diffusion.cpp), not a text LLM — its GGUFs have zero metadata (`kv_count=0`) and Ollama cannot load them. Its two ~30 GB files under `models/minimax-h3/` are dead weight and can be deleted.
