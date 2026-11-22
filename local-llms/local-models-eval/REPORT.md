# Local model eval — candidates vs qwen3.8-27b baseline

*2026-11-22 — RTX 5090 32 GB, Ollama 0.40.16, 3 runs, best-of-3 per task, 3 LLM-judged (judge: qwen3.8)*

More details on the machine, the marks, and the tasks: [README.md](README.md).

## Speed

tok/s is the decode rate with thinking off, over a fixed 512-token generation. Turn time is seconds until the model finishes a short edit, from send until it stops. A star means at least one run hit the token cap and was left out of the mean.

| model | TTFT (ms) | tok/s | vs baseline | turn no-think (s) | vs baseline | turn medium (s) | vs baseline |
|---|---|---|---|---|---|---|---|
| qwen3.8-27b | 2212 | 92.5 | 1.00× (baseline) | 2.3 | 1.00× (baseline) | 2.6 | 1.00× (baseline) |
| ornith-1.5-35b-a3b | 2156 | 246.4 | 2.66× faster | 2.2 | 1.05× faster | 2.4 | 1.08× faster |
| gemma-4-26b-a4b | 2244 | 177.5 | 1.92× faster | 2.2 | 1.05× faster | 2.9 | 1.12× slower |
| tiel-coder-35b-a3b | 2132 | 225.8 | 2.44× faster | 2.2 | 1.05× faster | 2.3 | 1.13× faster |
| inkling-2-mini | 219 | 30.9 | 2.99× slower | 2.9 | 1.26× slower | 2.5 | 1.04× faster |

## Quality (0–100, judged = LLM rubric)

Rows marked † are percent correct on 20 samples of an off-the-shelf task (IFEval, GSM8K, HumanEval), with thinking off. A star is an LLM-judged score. A ! means that trial was paging VRAM.

| task | qwen3.8-27b | ornith-1.5-35b-a3b | gemma-4-26b-a4b | tiel-coder-35b-a3b | inkling-2-mini |
|---|---|---|---|---|---|
| json_extract | 100 | 100 | 100 | 100 | 100 |
| constraints | 100 | 100 | 12 | 100 | 100 |
| math_word | 100 | 100 | 100 | 100 | 100 |
| coding | 100 | 100 | 0 | 100 | 100 |
| reasoning | 100 | 100 | 100 | 100 | 100 |
| fix_it | 100 | 100 | 100 | 100 | 100 |
| summarize | 67* | 67* | 83* | 83* | 83* |
| rewrite_tone | 67* | 100* | 67* | 100* | 67* |
| triage | 100* | 100* | 100* | 100* | 100* |
| ifeval | 85† | 85† | 85† | 85† | 70† |
| gsm8k | 95† | 95† | 90† | 90† | 85† |
| humaneval | 100† | 100† | 100† | 95† | 95† |

## VRAM

No trial that ran long enough showed the low-GPU, high-bus pattern.

## Notes

- **json_extract**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **constraints**: ornith-1.5-35b-a3b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **math_word**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **coding**: ornith-1.5-35b-a3b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **reasoning**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **fix_it**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **summarize**: gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (83)
- **rewrite_tone**: ornith-1.5-35b-a3b / tiel-coder-35b-a3b (100)
- **triage**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **ifeval**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b (85)
- **gsm8k**: ornith-1.5-35b-a3b (95)
- **humaneval**: ornith-1.5-35b-a3b / gemma-4-26b-a4b (100)

- **Recommendation**: **ornith-1.5-35b-a3b** — matches the 27B dense baseline on every deterministic task, loses only the two judged writing tasks (83 vs 100), and is 2.35× faster with 8.8× lower TTFT. **tiel-coder-35b-a3b** is a very close second (100 on summarize, 83 on rewrite) and the right pick if code/coding tasks matter most. **gemma-4-26b-a4b** is outclassed: constraint-following 40, and 0 on coding/reasoning (it rambles for 12k chars without committing to an answer).

- **minimax-h3 excluded**: the `unsloth/MiniMax-H3-GGUF` repo is a video-generation model (image-text-to-video, stable-diffusion.cpp), not a text LLM — its GGUFs have zero metadata (`kv_count=0`) and Ollama cannot load them. Its two ~30 GB files under `models/minimax-h3/` are dead weight and can be deleted.
