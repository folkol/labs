# /// script
# requires-python = ">=3.10"
# dependencies = ["huggingface_hub", "requests", "pyyaml"]
# ///
"""Linux run of the same eval, with Inkling on the official checkpoint.

Boot the Linux install on this machine, then from this directory:

    uv run e2e-linux.py

Inkling is thinkingmachines/Inkling-2-Mini (NVFP4/MXFP8) served by SGLang.
The other models stay on llama-server and the GGUF files in config-linux.yaml.
Writes results-linux.json and REPORT-linux.md.
Leaves config.yaml, results.json, and REPORT.md alone.

Needs nvidia-smi, and either Docker with the NVIDIA container toolkit or
SGLANG_PYTHON pointing at an environment that can import sglang. The first
run pulls lmsysorg/sglang:inkling-2-mini and the 20 GB checkpoint.
The qwen GGUF is the blob on the Windows E: drive; mount that disk first.
"""
import json
import os
import re
import shutil
import subprocess
import sys
import tarfile
import time
import urllib.request
from pathlib import Path

try:
    import yaml
except ImportError:
    raise SystemExit("missing 'yaml' — run with: uv run e2e-linux.py") from None

HERE = Path(__file__).resolve().parent
CFG = HERE / "config-linux.yaml"
RESULTS = HERE / "results-linux.json"
REPORT = HERE / "REPORT-linux.md"
BIN = HERE / ".bin" / "linux"
LOGS = HERE / "logs"
IMAGE = "lmsysorg/sglang:inkling-2-mini"


def log(msg):
    print(f"\n[e2e-linux] {msg}", flush=True)


def require_linux():
    if sys.platform == "linux":
        return
    raise SystemExit(
        "This is the Linux run. Boot the Linux install on this machine and run:\n"
        "  uv run e2e-linux.py\n"
        "It writes results-linux.json and REPORT-linux.md."
    )


def step_llama():
    exe = next(BIN.rglob("llama-server"), None) if BIN.exists() else None
    if exe and exe.is_file() and not exe.name.endswith(".exe"):
        log(f"llama.cpp present: {exe}")
        return exe
    found = shutil.which("llama-server")
    if found and not found.endswith(".exe"):
        log(f"llama-server on PATH: {found}")
        return Path(found)
    log("fetching latest llama.cpp (Ubuntu CUDA build)")
    req = urllib.request.Request(
        "https://api.github.com/repos/ggml-org/llama.cpp/releases?per_page=15",
        headers={"User-Agent": "eval-e2e"},
    )
    with urllib.request.urlopen(req, timeout=30) as r:
        releases = json.load(r)
    for rel in releases:
        builds = [a for a in rel["assets"]
                  if "bin-ubuntu-cuda" in a["name"] and a["name"].endswith("-x64.tar.gz")]
        if not builds:
            continue

        def key(a):
            v = re.search(r"cuda-(\d+)\.(\d+)", a["name"])
            return (0 if a["name"].startswith("cudart-") else 1,
                    (int(v.group(1)), int(v.group(2))) if v else (0, 0))

        builds.sort(key=key)
        asset = builds[-1]
        break
    else:
        raise SystemExit("no Ubuntu CUDA build found in recent llama.cpp releases")
    log(f"  {rel['tag_name']} -> {asset['name']}")
    BIN.mkdir(parents=True, exist_ok=True)
    archive = BIN / "llama.tar.gz"
    with urllib.request.urlopen(asset["browser_download_url"], timeout=60) as r, open(archive, "wb") as f:
        total = int(r.headers.get("Content-Length") or 0)
        done = 0
        while chunk := r.read(1 << 20):
            f.write(chunk)
            done += len(chunk)
            print(f"\r    {done/1e6:.0f}/{total/1e6:.0f} MB", end="", flush=True)
    print()
    log("extracting")
    with tarfile.open(archive) as tar:
        try:
            tar.extractall(BIN, filter="data")
        except TypeError:
            tar.extractall(BIN)
    archive.unlink()
    exe = next((p for p in BIN.rglob("llama-server") if p.is_file() and not p.name.endswith(".exe")), None)
    if not exe:
        raise SystemExit(f"llama-server not found under {BIN}")
    exe.chmod(exe.stat().st_mode | 0o755)
    log(f"  llama-server: {exe}")
    return exe


def translate_windows(path_str):
    """A Windows path as seen from a dual-boot Linux mount, if the file is there."""
    match = re.match(r"^([A-Za-z]):[\\/](.*)$", path_str)
    if not match:
        return None
    drive, rest = match.group(1).lower(), match.group(2).replace("\\", "/").lstrip("/")
    roots = [Path(f"/mnt/{drive}"), Path(f"/mnt/{drive.upper()}")]
    for base in (Path("/mnt"), Path("/media"), Path("/run/media")):
        if not base.is_dir():
            continue
        for child in base.iterdir():
            roots.append(child)
            if child.is_dir():
                for sub in child.iterdir():
                    if sub.is_dir():
                        roots.append(sub)
    name = Path(rest).name
    seen = set()
    for root in roots:
        key = str(root)
        if key in seen or not root.is_dir():
            continue
        seen.add(key)
        direct = root / rest
        if direct.is_file():
            return direct.resolve()
        blob = root / ".ollama" / "models" / "blobs" / name
        if blob.is_file():
            return blob.resolve()
    return None


def existing_file(path_str):
    raw = Path(path_str)
    if raw.is_file():
        return raw.resolve()
    if not raw.is_absolute():
        local = HERE / raw
        if local.is_file():
            return local.resolve()
    return translate_windows(path_str)


def resolve_models(cfg):
    import e2e
    for m in cfg["models"]:
        if m.get("runner") == "sglang":
            log(f"{m['name']}: official checkpoint {m['repo']} via SGLang")
            continue
        found = existing_file(m["gguf"]) if m.get("gguf") else None
        if found:
            m["gguf"] = str(found)
            log(f"{m['name']}: {found.name} ({found.stat().st_size/1e9:.1f} GB)")
            continue
        path, where = e2e.resolve_gguf(m, cfg)
        if not path:
            hint = ""
            if m.get("gguf") and re.match(r"^[A-Za-z]:", m["gguf"]):
                hint = f"\nMount the Windows drive that holds {m['gguf']} (often /mnt/e/...)."
            raise SystemExit(f"no gguf for {m['name']} visible from Linux.{hint}")
        m["gguf"] = str(Path(path).resolve())
        log(f"{m['name']}: {Path(path).name} from {where}")


def ollama_tags():
    try:
        with urllib.request.urlopen("http://127.0.0.1:11434/api/tags", timeout=3) as r:
            return json.load(r).get("models") or []
    except Exception:
        return None


def configure_judge(cfg):
    """Prefer the same Ollama qwen3.8 judge as the Windows run."""
    tags = ollama_tags()
    if tags is not None:
        cfg["ollama"] = {"base_url": "http://127.0.0.1:11434/v1"}
    else:
        cfg.pop("ollama", None)
    names = [t.get("name") or "" for t in (tags or [])]
    if any(n == "qwen3.8" or n.startswith("qwen3.8:") for n in names):
        cfg["judge"] = {"base_url": "http://127.0.0.1:11434/v1", "model": "qwen3.8:latest"}
        log("judge: Ollama qwen3.8:latest")
        return "Ollama"
    qwen = next(m for m in cfg["models"] if m["name"] == "qwen3.8-27b")
    cfg["judge"] = {
        "base_url": "http://127.0.0.1:8099/v1",
        "gguf": qwen["gguf"],
        "model": "qwen3.8-27b",
    }
    log("judge: llama-server on the qwen GGUF (Ollama has no qwen3.8)")
    return "llama-server"


def prefetch_checkpoint(repo):
    from huggingface_hub import snapshot_download
    log(f"resolving {repo} in the Hugging Face cache")
    snapshot_download(repo_id=repo)


def preflight_sglang():
    if os.environ.get("SGLANG_PYTHON"):
        log(f"SGLang python: {os.environ['SGLANG_PYTHON']}")
        return
    if shutil.which("docker") is None:
        raise SystemExit(
            "Docker is not on PATH. Install Docker and the NVIDIA container toolkit,\n"
            f"or set SGLANG_PYTHON to a python that can import sglang.\n"
            f"The verified image is {IMAGE}."
        )
    info = subprocess.run(["docker", "info"], capture_output=True, text=True)
    if info.returncode != 0:
        raise SystemExit("docker is installed but the daemon is not usable:\n" + (info.stderr or info.stdout))
    have = subprocess.run(["docker", "image", "inspect", IMAGE],
                          stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    if have.returncode != 0:
        log(f"pulling {IMAGE}")
        subprocess.run(["docker", "pull", IMAGE], check=True)
    else:
        log(f"image present: {IMAGE}")


def main():
    t0 = time.time()
    require_linux()
    if shutil.which("nvidia-smi") is None:
        raise SystemExit("nvidia-smi is not on PATH. The 5090 driver is not available in this boot.")
    cfg = yaml.safe_load(CFG.read_text(encoding="utf-8"))
    resolve_models(cfg)
    judge_how = configure_judge(cfg)
    CFG.write_text(yaml.safe_dump(cfg, sort_keys=False), encoding="utf-8")
    exe = step_llama()
    inkling = next(m for m in cfg["models"] if m.get("runner") == "sglang")
    prefetch_checkpoint(inkling["repo"])
    preflight_sglang()

    def py(*args):
        cmd = [sys.executable, "run_eval.py", "--config", str(CFG), "--llama", str(exe),
               "--out", str(RESULTS), *args]
        subprocess.run(cmd, cwd=HERE, check=True)

    log("phase 1: speed + deterministic tasks")
    py("--no-judge")

    proc = None
    if judge_how == "llama-server":
        import e2e
        proc = e2e.step_judge(cfg, exe)
        with urllib.request.urlopen(cfg["judge"]["base_url"].rstrip("/") + "/models", timeout=5) as r:
            cfg["judge"]["model"] = json.load(r)["data"][0]["id"]
        CFG.write_text(yaml.safe_dump(cfg, sort_keys=False), encoding="utf-8")
    try:
        log("phase 2: judging rubric tasks")
        py("--judge-only")
    finally:
        if proc:
            proc.terminate()
            try:
                proc.wait(timeout=15)
            except subprocess.TimeoutExpired:
                proc.kill()

    log("phase 3: IFEval, GSM8K, HumanEval")
    from external_evals import run_external
    run_external([
        "--config", str(CFG),
        "--out", str(RESULTS),
        "--logs", str(HERE / "external_logs_linux"),
        "--llama", str(exe),
    ])

    note = (
        "RTX 5090 32 GB, Linux, llama-server for the GGUF models, "
        "SGLang serving thinkingmachines/Inkling-2-Mini (NVFP4/MXFP8, plain decode), "
        f"3 runs, best-of-3, judge: qwen3.8 via {judge_how}"
    )
    log("writing REPORT-linux.md")
    subprocess.run(
        [sys.executable, str(HERE / "report.py"),
         "--results", str(RESULTS), "--out", str(REPORT), "--note", note],
        cwd=HERE, check=True,
    )
    log(f"done in {(time.time()-t0)/60:.0f} min — see {REPORT}")


if __name__ == "__main__":
    main()
