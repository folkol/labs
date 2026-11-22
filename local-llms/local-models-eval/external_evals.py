# /// script
# requires-python = ">=3.10"
# dependencies = [
#   "inspect-ai",
#   "inspect-evals",
#   "openai>=3.4.0",
#   "pyyaml",
#   "instruction_following_eval @ git+https://github.com/josejg/instruction_following_eval",
# ]
# ///
r"""IFEval, GSM8K, and HumanEval via Inspect.

A pass check for basic instruction-following, word problems, and short
Python functions. Not a leaderboard run: default is 20 samples per task.

Standalone (installs Inspect into its own env, does not touch e2e):

  uv run external_evals.py
  uv run external_evals.py --models qwen3.8-27b --limit 20
  uv run external_evals.py --skip-humaneval

HumanEval runs generated code. The default sandbox is "local", so that
code runs on this machine. Pass --sandbox docker if Docker is available.

e2e.py calls run_external() as phase 3. That re-invokes this file under
uv, so Inspect stays out of e2e's own dependencies.
"""
import argparse
import importlib.util
import json
import os

from gpu_watch import GpuWatch, warning_text
import shutil
import subprocess
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

import yaml

HERE = Path(__file__).resolve().parent
CFG = HERE / "config.yaml"
HARNESS_RESULTS = HERE / "results.json"
LOGS = HERE / "external_logs"
TASK_NAMES = ("ifeval", "gsm8k", "humaneval")
LLAMA_PORT = 8195


def run_external(extra_args=None):
    """Entry point for e2e. Re-runs this file under uv when Inspect is absent."""
    if importlib.util.find_spec("inspect_ai") is None:
        cmd = ["uv", "run", str(Path(__file__).resolve()), *(extra_args or [])]
        subprocess.run(cmd, cwd=HERE, check=True)
        return
    main(extra_args)


def main(argv=None):
    global CFG, HARNESS_RESULTS, LOGS
    ap = argparse.ArgumentParser(description="IFEval, GSM8K, and HumanEval")
    ap.add_argument("--config", default=str(CFG))
    ap.add_argument("--models", nargs="*", help="subset of model names in config.yaml")
    ap.add_argument("--limit", type=int, default=20, help="samples per task (default 20)")
    ap.add_argument("--skip-humaneval", action="store_true")
    ap.add_argument("--sandbox", default="local", help="HumanEval sandbox: local or docker")
    ap.add_argument("--llama", default=None, help="llama-server executable")
    ap.add_argument("--out", default=str(HARNESS_RESULTS), help="harness results json to update")
    ap.add_argument("--logs", default=str(LOGS), help="directory for Inspect logs")
    ap.add_argument("--rescore", action="store_true",
                    help="fill the results json from logs, without running models")
    args = ap.parse_args(argv)
    CFG = Path(args.config)
    HARNESS_RESULTS = Path(args.out)
    LOGS = Path(args.logs)
    if args.rescore:
        rescore_logs()
        return

    cfg = yaml.safe_load(Path(args.config).read_text(encoding="utf-8"))
    models = cfg["models"]
    if args.models:
        wanted = set(args.models)
        models = [m for m in models if m["name"] in wanted]
        missing = wanted - {m["name"] for m in models}
        if missing:
            raise SystemExit(f"unknown models: {', '.join(sorted(missing))}")
    if not models:
        raise SystemExit("no models selected")

    os.environ.setdefault("OPENAI_API_KEY", "ollama")
    tasks = load_tasks(include_humaneval=not args.skip_humaneval, sandbox=args.sandbox)
    rows = []
    for m in models:
        print(f"\n=== {m['name']} ===", flush=True)
        row = eval_model(m, cfg, tasks, args)
        rows.append(row)
        merge_into_harness(args.limit, rows)
    print_table(rows)


def load_tasks(include_humaneval, sandbox):
    try:
        from inspect_evals.gsm8k import gsm8k
        from inspect_evals.ifeval import ifeval
    except ImportError as e:
        raise SystemExit(
            "inspect-evals is not installed. Run: uv run external_evals.py"
        ) from e
    tasks = [("ifeval", ifeval()), ("gsm8k", gsm8k())]
    if include_humaneval:
        from inspect_evals.humaneval import humaneval
        tasks.append(("humaneval", humaneval(sandbox=sandbox)))
    return tasks


def eval_model(m, cfg, tasks, args):
    from inspect_ai import eval as inspect_eval

    proc = None
    try:
        unload_ollama(cfg)
        if m.get("runner") == "sglang":
            from official_server import start_official
            proc, base_url, model_id = start_official(m, cfg)
        elif is_llama_server(m):
            exe = args.llama or find_llama()
            proc, base_url, model_id = start_llama(exe, m, cfg)
        else:
            base_url = (m.get("base_url") or cfg["ollama"]["base_url"]).rstrip("/")
            model_id = m.get("id") or m.get("ollama_name") or m["name"]
        log_dir = LOGS / safe_name(m["name"])
        log_dir.mkdir(parents=True, exist_ok=True)
        scores = {}
        with GpuWatch() as gpu:
            for task_name, task in tasks:
                print(f"  {task_name} ...", flush=True)
                try:
                    with gpu.span(task_name):
                        logs = inspect_eval(
                            task,
                            model=f"openai/{model_id}",
                            model_base_url=base_url,
                            reasoning_effort="none",
                            temperature=0,
                            max_tokens=2048,
                            max_connections=1,
                            limit=args.limit,
                            log_dir=str(log_dir),
                            fail_on_error=False,
                            # "full" redraws the whole window. cmd.exe flickers.
                            display="plain",
                        )
                except Exception as e:
                    scores[task_name] = {"accuracy": None, "error": str(e)}
                    print(f"  {task_name}: failed ({_plain(e)})", flush=True)
                    continue
                scores[task_name] = score_of(logs, task_name)
                acc = scores[task_name]["accuracy"]
                line = f"  {task_name}: " + ("—" if acc is None else f"{acc:.0%}")
                info = gpu.spilled(task_name)
                if info:
                    line += f"  {warning_text(info)}"
                print(line, flush=True)
            vram = gpu.results
        return {"name": m["name"], "model_id": model_id, "tasks": scores, "vram": vram}
    finally:
        if proc:
            stop_server(proc)
        unload_ollama(cfg)


# IFEval has no metric named "accuracy". Prompt-level strict is the usual headline number.
METRIC_FOR = {"ifeval": "prompt_strict_acc"}


def score_of(logs, task_name):
    log = logs[0] if isinstance(logs, list) else logs
    status = getattr(log, "status", None)
    if status and status != "success":
        err = getattr(log, "error", None)
        return {"accuracy": None, "error": str(err or status), "metric": None}
    want = METRIC_FOR.get(task_name, "accuracy")
    results = getattr(log, "results", None)
    scores = getattr(results, "scores", None) or []
    for score in scores:
        metrics = getattr(score, "metrics", {}) or {}
        acc = metrics.get(want)
        if acc is not None:
            return {"accuracy": round(float(acc.value), 4), "error": None, "metric": want}
    return {"accuracy": None, "error": f"no {want} metric for {task_name}", "metric": want}


def rescore_logs():
    """Read finished .eval logs into results.json. Safe to run after a live pass."""
    from inspect_ai.log import read_eval_log

    if not HARNESS_RESULTS.exists():
        raise SystemExit(f"no {HARNESS_RESULTS.name}")
    cfg = yaml.safe_load(CFG.read_text(encoding="utf-8"))
    data = json.loads(HARNESS_RESULTS.read_text(encoding="utf-8"))
    by_name = {r["name"]: r for r in data}
    filled = 0
    for m in cfg["models"]:
        target = by_name.get(m["name"])
        folder = LOGS / safe_name(m["name"])
        if target is None or not folder.exists():
            continue
        latest = {}
        for path in folder.glob("*.eval"):
            for task_name in TASK_NAMES:
                if f"_{task_name}_" in path.name and (
                    task_name not in latest or path.stat().st_mtime > latest[task_name].stat().st_mtime
                ):
                    latest[task_name] = path
        for task_name, path in latest.items():
            log = read_eval_log(str(path))
            info = score_of(log, task_name)
            if info["accuracy"] is None:
                print(f"  {m['name']}/{task_name}: {info['error']}", flush=True)
                continue
            n = len(getattr(log, "samples", None) or [])
            target.setdefault("tasks", {})[task_name] = {
                "score": round(info["accuracy"] * 100),
                "details": {"samples": n or None, "error": None, "metric": info["metric"]},
            }
            print(f"  {m['name']}/{task_name}: {info['accuracy']:.0%}", flush=True)
            filled += 1
    HARNESS_RESULTS.write_text(json.dumps(data, indent=2), encoding="utf-8")
    print(f"updated {HARNESS_RESULTS} ({filled} scores)", flush=True)


def is_llama_server(m):
    if m.get("runner") in ("sglang", "ollama"):
        return False
    return m.get("runner") == "llama_server" or not (m.get("base_url") and (m.get("id") or m.get("ollama_name")))


def find_llama():
    found = shutil.which("llama-server")
    if found:
        return found
    hits = list((HERE / ".bin").rglob("llama-server.exe")) + list((HERE / ".bin").rglob("llama-server"))
    if hits:
        return str(hits[0])
    raise SystemExit("llama-server not on PATH and not under .bin/")


def start_llama(exe, m, cfg):
    gguf = Path(m["gguf"])
    if not gguf.is_absolute():
        gguf = HERE / gguf
    if not gguf.exists():
        raise SystemExit(f"missing gguf: {gguf}")
    server = cfg.get("server") or {}
    LOGS.mkdir(parents=True, exist_ok=True)
    logf = open(LOGS / f"llama_{LLAMA_PORT}.log", "ab")
    proc = subprocess.Popen(
        [exe, "-m", str(gguf), "-ngl", "99", "-c", str(server.get("ctx", 16384)),
         "-t", str(server.get("threads", 12)), "--port", str(LLAMA_PORT),
         "--no-webui", "--reasoning-effort", "none"],
        stdout=logf, stderr=subprocess.STDOUT,
    )
    base_url = f"http://127.0.0.1:{LLAMA_PORT}/v1"
    for _ in range(180):
        if proc.poll() is not None:
            raise RuntimeError(f"llama-server exited; see {LOGS / f'llama_{LLAMA_PORT}.log'}")
        try:
            data = http_json(f"{base_url}/models")
            model_id = data["data"][0]["id"]
            return proc, base_url, model_id
        except (urllib.error.URLError, KeyError, IndexError, TimeoutError):
            time.sleep(1)
    proc.kill()
    raise RuntimeError(f"llama-server on port {LLAMA_PORT} did not become ready")


def stop_server(proc):
    proc.terminate()
    try:
        proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        proc.kill()


def ollama_host(cfg):
    base = ((cfg.get("ollama") or {}).get("base_url") or "").rstrip("/")
    if base.endswith("/v1"):
        base = base[:-3]
    return base


def unload_ollama(cfg):
    host = ollama_host(cfg)
    if not host:
        return
    try:
        loaded = http_json(f"{host}/api/ps").get("models") or []
    except (urllib.error.URLError, TimeoutError):
        return
    for m in loaded:
        name = m.get("name") or m.get("model")
        if not name:
            continue
        print(f"  unloading {name}", flush=True)
        try:
            http_json(f"{host}/api/generate", {"model": name, "keep_alive": 0}, timeout=60)
        except (urllib.error.URLError, TimeoutError):
            pass


def http_json(url, body=None, timeout=5):
    data = None if body is None else json.dumps(body).encode()
    req = urllib.request.Request(url, data=data, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=timeout) as r:
        return json.load(r)


def merge_into_harness(limit, rows):
    """Fold sampled scores into results.json so report.py can print one table."""
    if not HARNESS_RESULTS.exists():
        raise SystemExit(f"no {HARNESS_RESULTS.name} yet — run the harness eval before this")
    data = json.loads(HARNESS_RESULTS.read_text(encoding="utf-8"))
    by_name = {r["name"]: r for r in data}
    for row in rows:
        target = by_name.get(row["name"])
        if target is None:
            print(f"  {row['name']} is not in {HARNESS_RESULTS.name}; skipped", flush=True)
            continue
        for task_name, info in row["tasks"].items():
            acc = info.get("accuracy")
            target.setdefault("tasks", {})[task_name] = {
                "score": None if acc is None else round(acc * 100),
                "details": {"samples": limit, "error": info.get("error")},
            }
        if row.get("vram"):
            target.setdefault("vram", {}).update(row["vram"])
    HARNESS_RESULTS.write_text(json.dumps(data, indent=2), encoding="utf-8")
    print(f"updated {HARNESS_RESULTS}", flush=True)


def print_table(rows):
    names = []
    for row in rows:
        for name in row["tasks"]:
            if name not in names:
                names.append(name)
    print("\n" + "=" * 72)
    print(f"{'model':<24}" + "".join(f"{n:>12}" for n in names))
    for row in rows:
        cells = []
        for name in names:
            acc = (row["tasks"].get(name) or {}).get("accuracy")
            cells.append(f"{'—':>12}" if acc is None else f"{acc:>11.0%}")
        print(f"{row['name']:<24}" + "".join(cells))


def _plain(exc):
    return str(exc).replace("[bold]", "").replace("[/bold]", "")


def safe_name(name):
    return "".join(c if c.isalnum() or c in "-._" else "_" for c in name)


if __name__ == "__main__":
    main()
