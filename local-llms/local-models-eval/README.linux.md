# Running the eval on Linux

`e2e-linux.py` is the Linux variant of `e2e.py`: same models, same phases, separate output.

```
uv run e2e-linux.py
```

It writes `results-linux.json`, `REPORT-linux.md` and `external_logs_linux/`. `results.json`, `REPORT.md` and `config.yaml` (which keeps the Windows paths) are not touched. The script works on `config-linux.yaml`, a copy of `config.yaml` that it regenerates on every run, so don't edit that one.

Tested on Ubuntu with an RTX 5090 (driver 595, CUDA 13.2), dual-booted with Windows.

## What the script does for you

- **llama.cpp:** downloads the latest `bin-ubuntu-cuda-*-x64.tar.gz` release into `.bin/linux/`.
- **CUDA libraries:** the llama.cpp CUDA build needs `libcudart.so.13` and `libcublas.so.13`. If the system has no CUDA toolkit, the script installs the `nvidia-cuda-runtime` and `nvidia-cublas` pip wheels into `.bin/cuda13/` and sets `LD_LIBRARY_PATH` for llama-server. No root needed.
- **Windows disks:** mounts any unmounted NTFS partition read-only through `udisksctl` (no root needed on a desktop session). Nothing is written to them.
- **Model weights:** looks in this order, and downloads from Hugging Face only if nothing matches:
  1. `models/<name>/` in this checkout
  2. the Windows Ollama store (`<disk>/.ollama/models`), found through its manifests
  3. the Windows project checkout (`Users/*/code/labs/local-llms/local-models-eval/models/`)
  4. the Windows Hugging Face cache (`Users/*/.cache/huggingface/hub`)
  5. local Ollama, the llama.cpp cache, the Linux Hugging Face cache
- **Judge:** uses the Ollama qwen3.8 if this machine has it. Otherwise it starts llama-server on the qwen GGUF on port 8099.

## Things that went wrong, and why

| Symptom | Cause | Fix |
|---|---|---|
| `PermissionError: /media/root` | the Windows-path scan hit an unreadable directory | removed the scan; lookup now ignores unreadable paths |
| `no gguf for qwen3.8-27b visible from Linux` | the qwen blob lives on the Windows `E:` disk, not mounted | mount read-only; the script finds it in place |
| 3 tok/s, TTFT 1.5 s, GPU at 600 MiB, llama-server at ~900% CPU | CUDA libs missing, so llama.cpp **silently fell back to the CPU** | `step_cuda_libs()`, see above |
| `ollama pull qwen3.8` fails with "requires a newer version" | Ollama 0.13.5 is too old | not fixed; the judge runs on llama-server instead. `curl -fsSL https://ollama.com/install.sh \| sh` needs root |
| 15 GB free on `/` | models are about 100 GB in total | read weights in place from the NTFS disks instead of downloading |

## Checking that the GPU is used

Do this in the first minutes of a run. A CPU fallback produces plausible but useless numbers.

```
nvidia-smi                                   # ~17 GB+ memory used while a model loads
.bin/linux/llama-b*/llama-server --list-devices   # must list CUDA0 (needs LD_LIBRARY_PATH from .bin/cuda13)
```

The first qwen line should report tok/s in the hundreds, as on Windows.

## Caveats

- Weights read from NTFS load more slowly the first time. After that they sit in the page cache, so it should not change scores. If speed numbers look off, copy the GGUFs into `models/<name>/` on the ext4 disk (about 100 GB) and re-run.
- Linux and Windows results are separate files on purpose. Compare them by hand; the report does not merge them.
- Leftovers you can delete: a partial `~/.cache/huggingface` download from an early aborted run.
