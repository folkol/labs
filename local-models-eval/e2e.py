# /// script
# requires-python = ">=3.10"
# dependencies = ["huggingface_hub", "requests", "pyyaml"]
# ///
"""End-to-end eval: deps -> llama.cpp -> model downloads -> run -> judge -> report.

Usage:  uv run e2e.py        (deps auto-installed by uv)
Requires: Python 3.10+, ~90 GB free disk, NVIDIA GPU + CUDA 12 runtime.
Idempotent: re-runs skip finished downloads.
"""
import json
import re
import subprocess
import sys
import time
import urllib.request
import zipfile
from pathlib import Path

try:
    import yaml
except ImportError:
    raise SystemExit("missing 'yaml' — run with: uv run e2e.py") from None

HERE = Path(__file__).resolve().parent
CFG = HERE / "config.yaml"
BIN = HERE / ".bin"
MODELS = HERE / "models"
LOGS = HERE / "logs"


def log(msg):
    print(f"\n[e2e] {msg}", flush=True)


def step_deps():
    missing = [pkg for mod, pkg in (("huggingface_hub", "huggingface_hub"),
                                    ("requests", "requests"), ("yaml", "pyyaml"))
               if __import__("importlib").util.find_spec(mod) is None]
    if missing:
        raise SystemExit(f"missing {missing} — run with: uv run e2e.py")


def step_llama():
    exe = next(BIN.rglob("llama-server.exe"), None)
    if exe:
        log(f"llama.cpp present: {exe}")
        return exe
    log("fetching latest llama.cpp (Windows CUDA build, from nightly releases)")
    req = urllib.request.Request(
        "https://api.github.com/repos/ggml-org/llama.cpp/releases?per_page=15",
        headers={"User-Agent": "eval-e2e"})
    with urllib.request.urlopen(req, timeout=30) as r:
        releases = json.load(r)
    # prefer self-contained CUDA build (no local toolkit needed), x64
    for rel in releases:
        x64 = [a for a in rel["assets"]
               if "bin-win-cuda" in a["name"] and a["name"].endswith("-x64.zip")]
        if not x64:
            continue
        def key(a):
            # self-contained build (no local CUDA toolkit needed), newest CUDA
            # (Blackwell needs >= 12.8). cudart- prefix = requires toolkit, avoid.
            v = re.search(r"cuda-(\d+)\.(\d+)", a["name"])
            return (0 if a["name"].startswith("cudart-") else 1,
                    (int(v.group(1)), int(v.group(2))) if v else (0, 0))
        x64.sort(key=key)
        asset = x64[-1]
        break
    else:
        raise SystemExit("no Windows CUDA build found in recent llama.cpp releases")
    log(f"  {rel['tag_name']} -> {asset['name']}")
    url = asset["browser_download_url"]
    log(f"downloading: {url}")
    BIN.mkdir(exist_ok=True)
    z = BIN / "llama.zip"
    with urllib.request.urlopen(url, timeout=60) as r, open(z, "wb") as f:
        total = int(r.headers.get("Content-Length") or 0)
        done = 0
        while chunk := r.read(1 << 20):
            f.write(chunk)
            done += len(chunk)
            print(f"\r    {done/1e6:.0f}/{total/1e6:.0f} MB", end="", flush=True)
    print()
    log("extracting")
    zipfile.ZipFile(z).extractall(BIN)
    z.unlink()
    exe = next(BIN.rglob("llama-server.exe"), None)
    if not exe:
        raise SystemExit(f"llama-server.exe not found under {BIN}")
    log(f"  llama-server: {exe.relative_to(HERE)}")
    return exe


def step_models(cfg):
    from huggingface_hub import snapshot_download
    for m in cfg["models"]:
        if m.get("ollama_name"):
            log(f"{m['name']}: using Ollama's {m['ollama_name']} — no download")
            continue
        d = MODELS / m["name"]
        d.mkdir(parents=True, exist_ok=True)
        files = list(d.glob("*.gguf"))
        if not files:
            log(f"downloading {m['repo']} ({m['include']}) -> {d.relative_to(HERE)}")
            snapshot_download(repo_id=m["repo"], allow_patterns=[m["include"]], local_dir=str(d))
            files = list(d.glob("*.gguf"))
        if not files:
            raise SystemExit(f"no gguf for {m['name']} in {d} — check repo/include in config.yaml")
        best = max(files, key=lambda p: p.stat().st_size)
        m["gguf"] = str(best.relative_to(HERE)).replace("\\", "/")
        log(f"{m['name']}: {best.name} ({best.stat().st_size/1e9:.1f} GB)")
    # judge gguf: explicit, or default to the baseline
    j = cfg.get("judge") or {}
    jg = Path(j.get("gguf", ""))
    if not jg.exists():
        base = next((x for x in cfg["models"] if x["name"] == "qwen3.8-27b"), None)
        if base:
            j["gguf"] = base["gguf"]
        elif not re.match(r"https?://(?!127\.0\.0\.1|localhost)", j.get("base_url", "")):
            raise SystemExit("no local judge gguf and judge.base_url isn't a remote API — set one in config.yaml")
    cfg["judge"] = j
    CFG.write_text(yaml.safe_dump(cfg, sort_keys=False), encoding="utf-8")
    log(f"config.yaml updated with resolved gguf paths")


def step_ollama(cfg):
    """Point each model at Ollama: reuse existing models, import GGUFs otherwise."""
    base = (cfg.get("ollama") or {}).get("base_url", "").rstrip("/")
    if not base:
        return
    try:
        with urllib.request.urlopen(base + "/models", timeout=5) as r:
            tags = json.load(r)["data"]
    except Exception as e:
        raise SystemExit(f"Ollama not reachable at {base}: {e}")
    have = {t["id"].split(":")[0] for t in tags}
    for m in cfg["models"]:
        if m.get("runner") == "llama_server":
            log(f"{m['name']}: llama-server runner — skipping Ollama")
            continue
        if m.get("ollama_name"):
            m["base_url"], m["id"] = base, m["ollama_name"]
            continue
        if m["name"] in have:
            m["base_url"] = base
            m["id"] = next(t["id"] for t in tags if t["id"].split(":")[0] == m["name"])
            log(f"{m['name']}: already in Ollama ({m['id']})")
            continue
        gguf = (HERE / m["gguf"]).resolve()
        mf = HERE / "Modelfile"
        mf.write_text(f"FROM {gguf}\n", encoding="utf-8")
        log(f"importing {m['name']} into Ollama from {gguf.name}")
        subprocess.run(["ollama", "create", m["name"], "--file", str(mf)], check=True)
        mf.unlink()
        m["base_url"], m["id"] = base, m["name"]
    # judge: local Ollama model needs no separate judge server
    j = cfg.get("judge") or {}
    if re.match(r"https?://(?:127\.0\.0\.1|localhost)", j.get("base_url", "")) and j.get("model"):
        log(f"judge: Ollama {j['model']}")
    cfg["ollama"] = {"base_url": base}
    CFG.write_text(yaml.safe_dump(cfg, sort_keys=False), encoding="utf-8")


def wait_ready(base_url, proc, what):
    for _ in range(120):
        try:
            with urllib.request.urlopen(base_url + "/models", timeout=1) as r:
                if r.status == 200:
                    return
        except Exception:
            pass
        if proc.poll() is not None:
            raise SystemExit(f"{what} died on startup — see {LOGS}/{what}.log")
        time.sleep(1)
    raise SystemExit(f"{what} did not become ready — see {LOGS}/{what}.log")


def step_judge(cfg, exe):
    """Start local judge if base_url is localhost; returns proc or None."""
    j = cfg["judge"]
    m = re.match(r"https?://(?:127\.0\.0\.1|localhost):(\d+)", j.get("base_url", ""))
    if not m:
        log("judge is a remote API — skipping local judge server")
        return None
    gguf = Path(j.get("gguf", ""))
    if not gguf.is_absolute():
        gguf = HERE / gguf
    if not gguf.exists():
        raise SystemExit(f"judge gguf missing: {gguf}")
    port = m.group(1)
    LOGS.mkdir(exist_ok=True)
    logf = open(LOGS / "judge.log", "ab")
    log(f"starting judge: {gguf.name} on port {port}")
    proc = subprocess.Popen(
        [str(exe), "-m", str(gguf), "-ngl", "99", "-c", "8192",
         "--port", port, "--no-webui"],
        stdout=logf, stderr=subprocess.STDOUT,
    )
    wait_ready(j["base_url"].rstrip("/"), proc, "judge")
    return proc


def main():
    t0 = time.time()
    cfg = yaml.safe_load(CFG.read_text(encoding="utf-8"))
    step_deps()
    step_models(cfg)
    ollama = bool((cfg.get("ollama") or {}).get("base_url"))
    exe = None if ollama else step_llama()
    if ollama:
        step_ollama(cfg)
    else:
        cfg = yaml.safe_load(CFG.read_text(encoding="utf-8"))

    def py(*args):
        cmd = [sys.executable, "run_eval.py", "--config", str(CFG), *args]
        if exe:
            cmd += ["--llama", str(exe)]
        subprocess.run(cmd, cwd=HERE, check=True)

    log("phase 1: speed + deterministic tasks (all candidates)")
    py("--no-judge")

    proc = None if ollama else step_judge(cfg, exe)
    try:
        log("phase 2: judging rubric tasks (judge in VRAM alone)")
        py("--judge-only")
    finally:
        if proc:
            proc.terminate()
            try:
                proc.wait(timeout=15)
            except subprocess.TimeoutExpired:
                proc.kill()

    log(f"done in {(time.time()-t0)/60:.0f} min — see {HERE/'results.json'}")


if __name__ == "__main__":
    main()
