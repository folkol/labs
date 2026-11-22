"""Serve the official Inkling 2 Mini checkpoint on Linux.

The weights are thinkingmachines/Inkling-2-Mini: one NVFP4/MXFP8 safetensors
file plus the bundled draft. SGLang's RTX 5090 recipe is the verified way
to run that file. This starts plain decode. The draft head is on disk and
is not loaded, so tok/s is the checkpoint itself, same idea as the Windows
GGUF run which has no draft model.

The verified 5090 context cap is 65536. The harness context (16384) is what
gets passed through, so this run and the Windows llama-server run share one
context setting.
"""
import os
import shutil
import subprocess
import time
import urllib.error
import urllib.request
from pathlib import Path

IMAGE = "lmsysorg/sglang:inkling-2-mini"
# SGLang's verified RTX 5090 Balanced recipe caps context here.
MAX_CTX = 65536
HERE = Path(__file__).resolve().parent
LOGS = HERE / "logs"


class ManagedProc:
    """A Popen the harness can stop, plus a docker container of the same server."""

    def __init__(self, proc, container=None):
        self._proc = proc
        self.container = container
        self._docker_stopped = False

    def poll(self):
        return self._proc.poll()

    def wait(self, timeout=None):
        return self._proc.wait(timeout=timeout)

    def terminate(self):
        self._stop(force=False)

    def kill(self):
        self._stop(force=True)

    def _stop(self, force):
        if self.container and not self._docker_stopped:
            self._docker_stopped = True
            cmd = ["docker", "kill" if force else "stop", self.container]
            if not force:
                cmd = ["docker", "stop", "-t", "8", self.container]
            subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        if self._proc.poll() is None:
            if force:
                self._proc.kill()
            else:
                self._proc.terminate()


def model_port(m, cfg):
    if "_idx" in m:
        idx = int(m["_idx"])
    else:
        idx = next(i for i, row in enumerate(cfg["models"]) if row["name"] == m["name"])
    return int(cfg["server"]["port_base"]) + idx


def hf_mount():
    """Host cache directory and the path it should have inside the container."""
    hub = os.environ.get("HUGGINGFACE_HUB_CACHE") or os.environ.get("HF_HUB_CACHE")
    if hub:
        return Path(hub), "/root/.cache/huggingface/hub"
    home = Path(os.environ.get("HF_HOME") or (Path.home() / ".cache" / "huggingface"))
    return home, "/root/.cache/huggingface"


def launch_args(model_path, served_name, port, ctx):
    if ctx > MAX_CTX:
        print(f"[sglang] context {ctx} is above the RTX 5090 cap {MAX_CTX}; using {MAX_CTX}", flush=True)
        ctx = MAX_CTX
    return [
        "-m", "sglang.launch_server",
        "--model-path", model_path,
        "--served-model-name", served_name,
        "--host", "0.0.0.0",
        "--port", str(port),
        "--tp", "1",
        "--attention-backend", "fa4",
        "--bf16-gemm-backend", "torch",
        "--dtype", "bfloat16",
        "--page-size", "128",
        "--context-length", str(ctx),
        "--reasoning-parser", "inkling",
        "--tool-call-parser", "inkling2",
    ]


def _tail(path, n=40):
    try:
        lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return ""
    return "\n".join(lines[-n:])


def _http_ok(url):
    try:
        with urllib.request.urlopen(url, timeout=2) as r:
            return r.status == 200
    except (urllib.error.URLError, TimeoutError, OSError):
        return False


def _model_id(base_url, fallback):
    try:
        with urllib.request.urlopen(base_url + "/models", timeout=5) as r:
            import json
            data = json.load(r)["data"]
        if data:
            return data[0]["id"]
    except (urllib.error.URLError, TimeoutError, OSError, KeyError, IndexError, ValueError):
        pass
    return fallback


def start_official(m, cfg):
    """Start SGLang for one model. Returns (proc, base_url, model_id)."""
    port = model_port(m, cfg)
    ctx = int(cfg["server"]["ctx"])
    repo = m["repo"]
    served = m.get("id") or m["name"]
    args = launch_args(repo, served, port, ctx)
    LOGS.mkdir(exist_ok=True)
    log_path = LOGS / f"sglang_{port}.log"
    logf = open(log_path, "ab")
    py = os.environ.get("SGLANG_PYTHON")
    container = None
    if py:
        cmd = [py, *args]
    else:
        if shutil.which("docker") is None:
            raise SystemExit(
                "SGLang is not available. Install Docker with the NVIDIA container toolkit, "
                f"or set SGLANG_PYTHON to a python that has sglang (image {IMAGE})."
            )
        container = f"eval-sglang-{port}"
        subprocess.run(["docker", "rm", "-f", container],
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        host_cache, container_cache = hf_mount()
        host_cache.mkdir(parents=True, exist_ok=True)
        env_key = "HUGGINGFACE_HUB_CACHE" if container_cache.endswith("/hub") else "HF_HOME"
        cmd = [
            "docker", "run", "--rm", "--name", container,
            "--gpus", "all", "--ipc", "host", "--shm-size", "8g",
            "-p", f"{port}:{port}",
            "-v", f"{host_cache}:{container_cache}",
            "-e", f"{env_key}={container_cache}",
            "--entrypoint", "python3",
            IMAGE,
            *args,
        ]
    print(f"[sglang] starting {repo} on port {port}", flush=True)
    print(f"[sglang] log: {log_path}", flush=True)
    proc = subprocess.Popen(cmd, stdout=logf, stderr=subprocess.STDOUT)
    handle = ManagedProc(proc, container)
    base = f"http://127.0.0.1:{port}/v1"
    deadline = time.time() + 1800
    next_note = time.time() + 20
    try:
        while time.time() < deadline:
            if handle.poll() is not None:
                raise RuntimeError(
                    f"SGLang exited. See {log_path}\n{_tail(log_path)}"
                )
            # /health_generate is ready only after weights are loaded.
            if _http_ok(f"http://127.0.0.1:{port}/health_generate"):
                return handle, base, _model_id(base, served)
            if time.time() >= next_note:
                print("[sglang] still loading the official checkpoint", flush=True)
                next_note = time.time() + 20
            time.sleep(2)
    except Exception:
        handle.terminate()
        raise
    handle.terminate()
    raise RuntimeError(f"SGLang on port {port} did not become ready. See {log_path}\n{_tail(log_path)}")
