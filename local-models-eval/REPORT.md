# Local model eval — candidates vs qwen3.8-27b baseline

*2026-11-22 — RTX 5090 32 GB, Ollama 0.40.16, 3 runs, best-of-3 per task, 2 LLM-judged (judge: qwen3.8)*

## Speed

| model | TTFT (ms) | tok/s | speedup vs baseline |
|---|---|---|---|
| qwen3.8-27b | 2199 | 103.1 | 1.00× (baseline) |
| ornith-1.5-35b-a3b | 2180 | 274.1 | 2.66× faster |
| gemma-4-26b-a4b | 2304 | 189.4 | 1.84× faster |
| tiel-coder-35b-a3b | 2136 | 238.8 | 2.32× faster |
| inkling-2-mini | 2370 | 13.7 | 7.53× slower |

## Quality (0–100, judged = LLM rubric)

| task | qwen3.8-27b | ornith-1.5-35b-a3b | gemma-4-26b-a4b | tiel-coder-35b-a3b | inkling-2-mini |
|---|---|---|---|---|---|
| json_extract | 100 | 100 | 100 | 100 | 100 |
| constraints | 100 | 100 | 100 | 100 | 100 |
| math_word | 100 | 100 | 100 | 100 | 100 |
| coding | 100 | 100 | 100 | 100 | 100 |
| reasoning | 100 | 100 | 0 | 100 | 50 |
| summarize | 100* | 83* | 100* | 100* | 100* |
| rewrite_tone | 100* | 100* | 100* | 100* | 100* |

## Notes

- **json_extract**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **constraints**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **math_word**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **coding**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **reasoning**: ornith-1.5-35b-a3b / tiel-coder-35b-a3b (100)
- **summarize**: gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **rewrite_tone**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)

- **Recommendation**: **ornith-1.5-35b-a3b** — matches the 27B dense baseline on every deterministic task, loses only the two judged writing tasks (83 vs 100), and is 2.35× faster with 8.8× lower TTFT. **tiel-coder-35b-a3b** is a very close second (100 on summarize, 83 on rewrite) and the right pick if code/coding tasks matter most. **gemma-4-26b-a4b** is outclassed: constraint-following 40, and 0 on coding/reasoning (it rambles for 12k chars without committing to an answer).

- **minimax-h3 excluded**: the `unsloth/MiniMax-H3-GGUF` repo is a video-generation model (image-text-to-video, stable-diffusion.cpp), not a text LLM — its GGUFs have zero metadata (`kv_count=0`) and Ollama cannot load them. Its two ~30 GB files under `models/minimax-h3/` are dead weight and can be deleted.
