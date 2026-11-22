# /// script
# requires-python = ">=3.10"
# dependencies = ["huggingface_hub", "requests", "pyyaml"]
# ///
"""Linux variant of e2e.py. Same models, same phases, separate results.

    uv run e2e-linux.py

Reads config.yaml but works on a copy (config-linux.yaml, regenerated every
run), so config.yaml keeps the Windows paths. Writes results-linux.json and
REPORT-linux.md; results.json and REPORT.md are untouched. Everything except
fetching the Linux llama.cpp build is reused from e2e.py.
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

import yaml

import e2e

HERE = e2e.HERE
SRC_CFG = HERE / "config.yaml"
CFG = HERE / "config-linux.yaml"
RESULTS = HERE / "results-linux.json"
REPORT = HERE / "REPORT-linux.md"
BIN = HERE / ".bin" / "linux"
log = e2e.log


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


def step_cuda_libs():
    """The llama.cpp CUDA build needs CUDA 13 runtime libs; without them it silently runs on CPU."""
    target = HERE / ".bin" / "cuda13"
    lib = next(target.rglob("libcudart.so.13"), None) if target.exists() else None
    if not lib and subprocess.run("ldconfig -p | grep -q libcudart.so.13", shell=True).returncode != 0:
        log("installing CUDA 13 runtime + cuBLAS (pip wheels) into .bin/cuda13")
        subprocess.run(["uv", "pip", "install", "--target", str(target),
                        "nvidia-cuda-runtime", "nvidia-cublas"], check=True)
        lib = next(target.rglob("libcudart.so.13"), None)
    if lib:
        os.environ["LD_LIBRARY_PATH"] = os.pathsep.join(
            [str(lib.parent), os.environ.get("LD_LIBRARY_PATH", "")]).rstrip(os.pathsep)
        log(f"CUDA libs: {lib.parent}")


def mount_windows_disks():
    """Mount unmounted NTFS partitions read-only (no root needed via udisks)."""
    out = subprocess.run(["lsblk", "-J", "-o", "NAME,FSTYPE,MOUNTPOINT"],
                         capture_output=True, text=True).stdout
    def walk(nodes):
        for n in nodes:
            yield n
            yield from walk(n.get("children") or [])
    for n in walk(json.loads(out or "{}").get("blockdevices", [])):
        if n.get("fstype") == "ntfs" and not n.get("mountpoint"):
            subprocess.run(["udisksctl", "mount", "-b", f"/dev/{n['name']}", "-o", "ro"],
                           capture_output=True)


def windows_weight(m):
    """A GGUF already on a Windows disk: its Ollama store or HF hub cache. Read-only use."""
    user = Path("/media") / os.environ.get("USER", "")
    for disk in user.glob("*"):
        names = {m["name"], m.get("ollama_name") or m["name"]}
        for mf in (disk / ".ollama/models/manifests").rglob("*") if (disk / ".ollama/models/manifests").is_dir() else []:
            if mf.is_file() and any(f"/{n}/" in mf.as_posix() for n in names):
                for layer in json.loads(mf.read_text()).get("layers", []):
                    if layer.get("mediaType", "").endswith("image.model"):
                        blob = disk / ".ollama/models/blobs" / layer["digest"].replace(":", "-")
                        if blob.is_file():
                            return blob, f"Windows Ollama store ({disk.name})"
        rel = m.get("gguf") or ""
        if rel and not re.match(r"^[A-Za-z]:|/", rel):  # project-relative, e.g. models/x/y.gguf
            for proj in disk.glob("Users/*/code/labs/local-llms/local-models-eval"):
                if (proj / rel).is_file():
                    return proj / rel, f"Windows checkout ({disk.name})"
        if m.get("repo"):
            for hub in disk.glob("Users/*/.cache/huggingface/hub"):
                snaps = hub / ("models--" + m["repo"].replace("/", "--")) / "snapshots"
                hits = [p for p in snaps.glob("*/" + (m.get("include") or "*.gguf"))
                        if p.is_file() and "mmproj" not in p.name] if snaps.is_dir() else []
                if hits:
                    return max(hits, key=lambda p: p.stat().st_size), f"Windows HF cache ({disk.name})"
    return None, ""


_resolve_gguf = e2e.resolve_gguf


def resolve_gguf(m, cfg):
    found = windows_weight(m)
    return found if found[0] else _resolve_gguf(m, cfg)


def main():
    t0 = time.time()
    if sys.platform != "linux":
        raise SystemExit("This is the Linux variant; on Windows use: uv run e2e.py")
    # e2e's steps read and rewrite e2e.CFG; point them at the Linux copy.
    cfg = yaml.safe_load(SRC_CFG.read_text(encoding="utf-8"))
    CFG.write_text(yaml.safe_dump(cfg, sort_keys=False), encoding="utf-8")
    e2e.CFG = CFG

    e2e.step_deps()
    mount_windows_disks()
    e2e.resolve_gguf = resolve_gguf
    e2e.step_models(cfg)
    exe = step_llama()
    step_cuda_libs()
    # Judge: the Ollama qwen3.8 if this machine has it, else llama-server on the
    # same qwen GGUF the baseline uses.
    tags = [t.get("name", "") for t in e2e.ollama_tags(cfg)]
    ollama = any(n == "qwen3.8" or n.startswith("qwen3.8:") for n in tags)
    if ollama:
        e2e.step_ollama(cfg)
    else:
        qwen = next(m for m in cfg["models"] if m["name"] == "qwen3.8-27b")
        cfg.pop("ollama", None)
        cfg["judge"] = {"base_url": "http://127.0.0.1:8099/v1", "gguf": qwen["gguf"],
                        "model": "qwen3.8-27b"}
        for m in cfg["models"]:
            m.pop("base_url", None)
        CFG.write_text(yaml.safe_dump(cfg, sort_keys=False), encoding="utf-8")
        log("judge: llama-server on the qwen GGUF (no qwen3.8 in local Ollama)")

    def py(*args):
        cmd = [sys.executable, "run_eval.py", "--config", str(CFG), "--llama", str(exe),
               "--out", str(RESULTS), *args]
        subprocess.run(cmd, cwd=HERE, check=True)

    log("phase 1: speed + deterministic tasks (all candidates)")
    py("--no-judge")

    proc = None if ollama else e2e.step_judge(cfg, exe)
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

    log("phase 3: IFEval, GSM8K, HumanEval (20 samples each)")
    from external_evals import run_external
    run_external(["--config", str(CFG), "--llama", str(exe), "--out", str(RESULTS),
                  "--logs", str(HERE / "external_logs_linux")])

    log("writing REPORT-linux.md")
    subprocess.run([sys.executable, str(HERE / "report.py"), "--results", str(RESULTS),
                    "--out", str(REPORT), "--note", "Linux run"], cwd=HERE, check=True)
    log(f"done in {(time.time()-t0)/60:.0f} min — see {REPORT}")


if __name__ == "__main__":
    main()
