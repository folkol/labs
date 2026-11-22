# Local model eval — candidates vs qwen3.8-27b baseline

*2026-11-22 — RTX 5090 32 GB, Ollama 0.40.16, 3 runs, best-of-3 per task, 2 LLM-judged (judge: qwen3.8)*

## Speed

tok/s counts every generated token, including the reasoning trace. Quality tasks run at medium effort with thinking on.

| model | TTFT no-think (ms) | tok/s no-think | vs baseline | TTFT medium (ms) | tok/s medium | vs baseline |
|---|---|---|---|---|---|---|
| qwen3.8-27b | 2222 | 100.0 | 1.00× (baseline) | 2244 | 103.1 | 1.00× (baseline) |
| ornith-1.5-35b-a3b | 2155 | 260.9 | 2.61× faster | 2119 | 262.9 | 2.55× faster |
| gemma-4-26b-a4b | 2298 | 181.9 | 1.82× faster | 2157 | 181.2 | 1.76× faster |
| tiel-coder-35b-a3b | 2141 | 215.3 | 2.15× faster | 2157 | 212.1 | 2.06× faster |
| inkling-2-mini | 251 | 28.1 | 3.56× slower | 283 | 30.0 | 3.44× slower |

## Quality (0–100, judged = LLM rubric)

| task | qwen3.8-27b | ornith-1.5-35b-a3b | gemma-4-26b-a4b | tiel-coder-35b-a3b | inkling-2-mini |
|---|---|---|---|---|---|
| json_extract | 100 | 100 | 100 | 100 | 100 |
| constraints | 100 | 100 | 100 | 100 | 100 |
| math_word | 100 | 100 | 100 | 100 | 100 |
| coding | 100 | 100 | 100 | 100 | 100 |
| reasoning | 100 | 100 | 0 | 100 | 100 |
| summarize | 100* | 67* | 83* | 33* | 33* |
| rewrite_tone | 100* | 100* | 100* | 67* | 100* |

## Notes

- **json_extract**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **constraints**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **math_word**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **coding**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **reasoning**: ornith-1.5-35b-a3b / tiel-coder-35b-a3b / inkling-2-mini (100)
- **summarize**: gemma-4-26b-a4b (83)
- **rewrite_tone**: ornith-1.5-35b-a3b / gemma-4-26b-a4b / inkling-2-mini (100)

- **Recommendation**: **ornith-1.5-35b-a3b** — matches the 27B dense baseline on every deterministic task, loses only the two judged writing tasks (83 vs 100), and is 2.35× faster with 8.8× lower TTFT. **tiel-coder-35b-a3b** is a very close second (100 on summarize, 83 on rewrite) and the right pick if code/coding tasks matter most. **gemma-4-26b-a4b** is outclassed: constraint-following 40, and 0 on coding/reasoning (it rambles for 12k chars without committing to an answer).

- **minimax-h3 excluded**: the `unsloth/MiniMax-H3-GGUF` repo is a video-generation model (image-text-to-video, stable-diffusion.cpp), not a text LLM — its GGUFs have zero metadata (`kv_count=0`) and Ollama cannot load them. Its two ~30 GB files under `models/minimax-h3/` are dead weight and can be deleted.
