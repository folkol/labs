r"""Run speed + quality eval across models, via llama.cpp servers.

Usage:
  python run_eval.py                 # all models in config.yaml
  python run_eval.py --models minimax-h3 gemma-4-26b-a4b
  python run_eval.py --no-judge      # skip rubric tasks (deterministic only)
  python run_eval.py --llama C:\tools\llama.cpp\build\bin\llama-server.exe
"""
import argparse
import json
import shutil
import statistics
import subprocess
import time
from pathlib import Path

import requests
import yaml

from gpu_watch import GpuWatch, warning_text
from tasks import TASKS

HERE = Path(__file__).parent

SPEED_PROMPT = (
    "Write a short story about a lighthouse keeper who receives a message "
    "bottled from the future. Aim for about 350 words, written as a single "
    "continuous narrative without headings."
)

# A finished agent step: the reply is a few lines, so extra time is thinking.
TURN_PROMPT = (
    "The function below subtracts when it should add.\n\n"
    "def add(a, b):\n"
    "    return a - b\n\n"
    "Reply with the corrected function only. Stop when it is done."
)


# ---------- server management ----------

def start_server(llama, gguf, port, ctx, threads):
    log = HERE / "logs" / f"server_{port}.log"
    log.parent.mkdir(exist_ok=True)
    logf = open(log, "ab")
    proc = subprocess.Popen(
        [llama, "-m", gguf, "-ngl", "99", "-c", str(ctx), "-t", str(threads),
         "--port", str(port), "--no-webui", "--metrics", "--reasoning-effort", "medium"],
        stdout=logf, stderr=subprocess.STDOUT,
    )
    url = f"http://127.0.0.1:{port}/v1"
    for _ in range(120):
        if proc.poll() is not None:
            raise RuntimeError(f"server died on startup; see {log}")
        try:
            if requests.get(f"{url}/models", timeout=1).status_code == 200:
                return proc, url
        except requests.RequestException:
            pass
        time.sleep(1)
    proc.kill()
    raise RuntimeError(f"server on port {port} did not become ready; see {log}")


def model_name(url):
    """llama.cpp serves models by GGUF filename; use what the server reports."""
    ids = requests.get(f"{url}/models", timeout=5).json()["data"]
    return ids[0]["id"] if ids else None


def stop_server(proc):
    proc.terminate()
    try:
        proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        proc.kill()


def ollama_host(cfg):
    """Native Ollama API root. config stores the OpenAI-compatible /v1 URL."""
    base = ((cfg.get("ollama") or {}).get("base_url") or "").rstrip("/")
    if base.endswith("/v1"):
        base = base[:-3]
    return base


def unload_ollama(cfg):
    """Unload every model Ollama is holding.

    Ollama keeps a model resident for keep_alive (default 5 minutes) after the
    last request. A later llama-server model never talks to Ollama, so the
    previous model stays in VRAM and the two page over the bus. Stopping here
    gives the next model the GPU to itself.
    """
    host = ollama_host(cfg)
    if not host:
        return
    try:
        loaded = requests.get(f"{host}/api/ps", timeout=5).json().get("models") or []
    except requests.RequestException as e:
        print(f"  ollama ps failed ({e}); VRAM may still be occupied")
        return
    names = [m.get("name") or m.get("model") for m in loaded]
    names = [n for n in names if n]
    for name in names:
        print(f"  unloading {name}")
        try:
            requests.post(f"{host}/api/generate",
                          json={"model": name, "keep_alive": 0}, timeout=60)
        except requests.RequestException as e:
            print(f"  unload {name} failed ({e})")
    if not names:
        return
    deadline = time.time() + 30
    still = loaded
    while time.time() < deadline:
        try:
            still = requests.get(f"{host}/api/ps", timeout=5).json().get("models") or []
        except requests.RequestException:
            return
        if not still:
            time.sleep(1)  # let the CUDA context actually release
            return
        time.sleep(0.5)
    left = ", ".join((m.get("name") or m.get("model") or "?") for m in still)
    raise RuntimeError(f"ollama still holding {left}")


# ---------- inference ----------

def chat_stream(url, model, prompt, max_tokens, keep_alive=None,
                thinking=True, reasoning_effort="medium", num_ctx=None,
                vendor="llama"):
    """Returns (text, ttft_s, tokens_per_s, n_tokens, truncated, total_s).

    Quality tasks use the defaults (thinking on, medium effort). The decode
    probe turns thinking off. Turn-time probes run both modes and use total_s,
    the wall clock from send until the model stops.
    """
    t0 = time.perf_counter()
    ttft = None
    n = 0
    finish = None
    parts, rparts = [], []
    # Ollama's OpenAI endpoint treats reasoning_effort "low"/"medium"/"high"
    # as thinking on. "none" is what disables it. `think` is ignored there.
    effort = reasoning_effort if thinking else "none"
    body = {"model": model, "max_tokens": max_tokens,
            "temperature": 0.2, "stream": True,
            "reasoning_effort": effort,
            "messages": [{"role": "user", "content": prompt}]}
    # SGLang rejects the Ollama-only fields. llama-server accepts `thinking`.
    if vendor == "ollama":
        body["thinking"] = thinking
        # -1 keeps an Ollama model loaded for this model run. The runner unloads
        # it explicitly afterwards.
        if keep_alive is not None:
            body["keep_alive"] = keep_alive
            body["think"] = effort if thinking else False
            if num_ctx:
                body["options"] = {"num_ctx": num_ctx}
    elif vendor == "llama":
        body["thinking"] = thinking
    # Slow models still have to be allowed to finish a long trace.
    timeout = max(600, max_tokens)
    with requests.post(
        f"{url}/chat/completions",
        json=body,
        stream=True, timeout=timeout,
    ) as resp:
        resp.raise_for_status()
        for line in resp.iter_lines():
            if not line:
                continue
            line = line.decode().removeprefix("data: ").strip()
            if line == "[DONE]":
                break
            choice = json.loads(line)["choices"][0]
            if choice.get("finish_reason"):
                finish = choice["finish_reason"]
            d = choice.get("delta") or {}
            # thinking models may emit `reasoning` deltas; count all tokens,
            # but prefer real content for the response text
            tok = d.get("content") or d.get("reasoning") or d.get("reasoning_content") or ""
            if tok:
                if ttft is None:
                    ttft = time.perf_counter() - t0
                n += 1
                if d.get("content"):
                    parts.append(tok)
                else:
                    # If the visible content is empty, the answer
                    # (e.g. "ANSWER: …") lives in the reasoning text.
                    rparts.append(tok)
        if ttft is None:
            raise RuntimeError(f"model streamed no tokens: {model}")
    total_s = time.perf_counter() - t0
    gen_t = total_s - ttft
    tps = n / gen_t if gen_t > 0.01 else float("inf")
    text = "".join(parts) or "".join(rparts)
    truncated = finish == "length" or (finish is None and n >= max_tokens)
    return text, ttft, tps, n, truncated, total_s


# ---------- scoring ----------

def score_deterministic(task, response):
    results = [(name, fn(response)) for name, fn in task["checks"]]
    passed = sum(1 for _, ok in results if ok)
    score = passed / len(results) * 100
    return score, results


JUDGE_SYS = (
    "You are a strict evaluator. Score each criterion 0 (fail), 1 (partial), "
    "or 2 (full). Respond with ONLY a JSON object: "
    '{"scores": {"<criterion>": <int>}}'
)


def score_judged(cfg, task, response):
    base = cfg["judge"]["base_url"].rstrip("/")
    jmodel = cfg["judge"].get("model") or ""
    body = {
        "model": jmodel or model_name(base),
        "thinking": False,
        "reasoning_effort": "medium",
        "temperature": 0,
        # judge is a thinking model: it emits ~300+ reasoning tokens before
        # the JSON even with thinking:False — budget must cover both
        "max_tokens": 2000,
        "messages": [
            {"role": "system", "content": JUDGE_SYS},
            {"role": "user", "content":
                "Task prompt:\n" + task["prompt"] + "\n\n"
                "Candidate response:\n" + response + "\n\n"
                "Criteria:\n" + "\n".join(f"- {n}: {d}" for n, d in task["criteria"])
                + "\n\nReturn the JSON scores."},
        ],
    }
    r = requests.post(f"{base}/chat/completions", json=body, timeout=120)
    r.raise_for_status()
    m = r.json()["choices"][0]["message"]
    txt = m.get("content") or m.get("reasoning") or ""
    if "{" not in txt:
        raise RuntimeError(f"judge returned no scores: {txt[:200]!r}")
    scores = json.loads(txt[txt.index("{"):txt.rindex("}") + 1])["scores"]
    total = sum(int(scores.get(n, 0)) for n, _ in task["criteria"])
    score = total / (2 * len(task["criteria"])) * 100
    return score, [(n, int(scores.get(n, 0))) for n, _ in task["criteria"]]


# ---------- per-model pipeline ----------

def uses_ollama(m):
    """Ollama only when this model is not pinned to another runner."""
    if m.get("runner") not in (None, "", "ollama"):
        return False
    return bool(m.get("base_url") and m.get("id"))


def gguf_path(m):
    path = Path(m["gguf"])
    if not path.is_absolute():
        path = HERE / path
    return str(path.resolve())


def eval_model(m, cfg, llama, no_judge):
    max_tok = cfg["max_tokens"]
    runs = cfg["runs"]
    # Free the GPU before this model allocates. Ollama will not do it on its
    # own when the next runner is llama-server.
    unload_ollama(cfg)
    ollama = uses_ollama(m)
    proc = None
    if ollama:
        url, name = m["base_url"].rstrip("/"), m["id"]
        vendor = "ollama"
        print(f"\n=== {m['name']} ({name} @ {url}) ===")
    elif m.get("runner") == "sglang":
        from official_server import start_official
        vendor = "sglang"
        proc, url, name = start_official(m, cfg)
        print(f"\n=== {m['name']} ({m['repo']} @ {url}) ===")
    else:
        vendor = "llama"
        port = cfg["server"]["port_base"] + m["_idx"]
        print(f"\n=== {m['name']} (port {port}) ===")
        proc, url = start_server(llama, m["gguf"], port, cfg["server"]["ctx"], cfg["server"]["threads"])
        name = model_name(url)
    # Hold an Ollama model until this run finishes; default keep_alive is 5m
    # and a long task list would otherwise reload mid-run.
    hold = -1 if ollama else None
    ctx = cfg["server"]["ctx"]
    out = {"name": m["name"], "gguf": m.get("gguf"), "tasks": {}, "speed": {}, "vram": {}}
    if vendor == "sglang":
        out["checkpoint"] = m["repo"]
        out["serve"] = "sglang NVFP4/MXFP8, plain decode"
    try:
        with GpuWatch() as gpu:
            # warmup (compiles hot loop, loads weights)
            chat_stream(url, name, "Say OK.", 8, keep_alive=hold,
                        thinking=False, reasoning_effort="low", num_ctx=ctx, vendor=vendor)

            # Decode rate, once. Thinking does not change tok/s, so this probe
            # stays thinking-off with a fixed token budget.
            ttps, tss = [], []
            for _ in range(runs):
                with gpu.span("tok/s"):
                    _, ttft, tps, *_ = chat_stream(
                        url, name, SPEED_PROMPT, cfg["speed_max_tokens"],
                        keep_alive=hold, thinking=False, reasoning_effort="low",
                        num_ctx=ctx, vendor=vendor)
                ttps.append(ttft)
                tss.append(tps)

            def measure_turn(thinking, effort, label):
                elapsed, tokens = [], []
                for _ in range(runs):
                    with gpu.span(label):
                        _, _, _, n, truncated, total_s = chat_stream(
                            url, name, TURN_PROMPT, cfg["turn_max_tokens"],
                            keep_alive=hold, thinking=thinking, reasoning_effort=effort,
                            num_ctx=ctx, vendor=vendor)
                    if truncated:
                        continue
                    elapsed.append(total_s)
                    tokens.append(n)
                if not elapsed:
                    return {"seconds_mean": None, "tokens_mean": None,
                            "finished": 0, "runs": runs}
                return {
                    "seconds_mean": round(statistics.mean(elapsed), 1),
                    "tokens_mean": round(statistics.mean(tokens)),
                    "finished": len(elapsed),
                    "runs": runs,
                }

            turn_off = measure_turn(False, "low", "turn no-think")
            turn_med = measure_turn(True, "medium", "turn medium")
            out["speed"] = {
                "ttft_ms_mean": round(statistics.mean(ttps) * 1000),
                "tok_per_s_mean": round(statistics.mean(tss), 1),
                "turn_no_thinking_low": turn_off,
                "turn_medium": turn_med,
            }
            print(f"  tok/s (thinking off): ttft={out['speed']['ttft_ms_mean']}ms  "
                  f"{out['speed']['tok_per_s_mean']} tok/s")

            def fmt_turn(block):
                if block["seconds_mean"] is None:
                    return f"hit {cfg['turn_max_tokens']} cap"
                note = "" if block["finished"] == block["runs"] else f" ({block['finished']}/{block['runs']} finished)"
                return f"{block['seconds_mean']}s, {block['tokens_mean']} tok{note}"

            print(f"  turn thinking off: {fmt_turn(turn_off)}")
            print(f"  turn medium:       {fmt_turn(turn_med)}")
            for label in ("tok/s", "turn no-think", "turn medium"):
                info = gpu.spilled(label)
                if info:
                    print(f"  {label}: {warning_text(info)}")

            # quality tasks
            for t in TASKS:
                best, details, first_resp = -1, None, None
                for i in range(runs):
                    with gpu.span(t["id"]):
                        resp, ttft, tps, *_ = chat_stream(
                            url, name, t["prompt"], max_tok, keep_alive=hold,
                            num_ctx=ctx, vendor=vendor)
                    if first_resp is None:
                        first_resp = resp
                    if "checks" in t:
                        score, details = score_deterministic(t, resp)
                    else:
                        score, details = (None, None) if no_judge else score_judged(cfg, t, resp)
                    if score is not None and score > best:
                        best, details = score, details
                td = {
                    "score": round(best) if best >= 0 else None,
                    "details": {k: v for k, v in (details or [])},
                }
                if "checks" not in t:
                    td["response"] = first_resp  # for --judge-only later
                out["tasks"][t["id"]] = td
                line = f"  {t['id']}: " + (f"{best:.0f}" if best >= 0 else "pending judge")
                info = gpu.spilled(t["id"])
                if info:
                    line += f"  {warning_text(info)}"
                print(line)
            out["vram"] = gpu.results
    finally:
        if proc:
            stop_server(proc)
        unload_ollama(cfg)
    return out


# ---------- main ----------

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--config", default=str(HERE / "config.yaml"))
    ap.add_argument("--models", nargs="*", help="subset of model names")
    ap.add_argument("--llama", default=shutil.which("llama-server") or "llama-server")
    ap.add_argument("--no-judge", action="store_true",
                    help="generate judged-task responses but defer scoring")
    ap.add_argument("--judge-only", action="store_true",
                    help="score saved responses in --out using the judge, then re-report")
    ap.add_argument("--out", default=str(HERE / "results.json"))
    args = ap.parse_args()

    cfg = yaml.safe_load(open(args.config, encoding="utf-8"))
    if args.judge_only:
        results = json.loads(Path(args.out).read_text(encoding="utf-8"))
        for r in results:
            for tid, td in r["tasks"].items():
                if td.get("score") is None and td.get("response"):
                    t = next(x for x in TASKS if x["id"] == tid)
                    s, det = score_judged(cfg, t, td["response"])
                    td["score"] = round(s)
                    td["details"] = {k: v for k, v in det}
                    print(f"  {r['name']}/{tid}: {s:.0f}")
        Path(args.out).write_text(json.dumps(results, indent=2), encoding="utf-8")
        print_report(results, args.out)
        return

    models = cfg["models"]
    if args.models:
        models = [m for m in models if m["name"] in args.models]
    if not models:
        raise SystemExit("no models matched; check --models names against config.yaml")
    for m in models:
        m["_idx"] = cfg["models"].index(m)
        if uses_ollama(m) or m.get("runner") == "sglang":
            continue
        if "gguf" not in m:
            raise SystemExit(f"{m['name']}: no gguf path — run e2e.py first (it downloads and fills this in)")
        m["gguf"] = gguf_path(m)
        if not Path(m["gguf"]).exists():
            raise SystemExit(f"missing gguf: {m['gguf']}")

    results = [eval_model(m, cfg, args.llama, args.no_judge) for m in models]
    Path(args.out).write_text(json.dumps(results, indent=2), encoding="utf-8")
    print_report(results, args.out)


def _turn_cell(block):
    if not block or block.get("seconds_mean") is None:
        text = "—"
    else:
        mark = "" if block["finished"] == block["runs"] else "*"
        text = f"{block['seconds_mean']:.1f}{mark}"
    return f"{text:>8}"


def print_report(results, out=None):
    sp0 = results[0]["speed"] if results else {}
    turns = "turn_medium" in sp0
    dual = "no_thinking_low" in sp0
    print("\n" + "=" * 78)
    if turns:
        print("tok/s: decode rate, thinking off. off/med: seconds until a short edit finishes")
        head = f"{'model':<20}{'tok/s':>8}{'off s':>8}{'med s':>8}"
    elif dual:
        print("speed: off = no thinking, low effort; med = thinking, medium effort")
        head = f"{'model':<20}{'off ms':>8}{'off t/s':>8}{'med ms':>8}{'med t/s':>8}"
    else:
        head = f"{'model':<20}{'ttft ms':>9}{'tok/s':>8}"
    print(head + "".join(f"{t['id'][:9]:>10}" for t in TASKS))
    for r in results:
        cells = []
        for t in TASKS:
            v = r["tasks"].get(t["id"], {}).get("score")
            cells.append(f"{'-':>6}" if v is None else f"{v:>5}")
        q = "  ".join(cells)
        sp = r["speed"]
        if turns:
            nums = (f"{sp['tok_per_s_mean']:>8}"
                    f"{_turn_cell(sp['turn_no_thinking_low'])}"
                    f"{_turn_cell(sp['turn_medium'])}")
        elif dual:
            low, med = sp["no_thinking_low"], sp["medium"]
            nums = (f"{low['ttft_ms_mean']:>8}{low['tok_per_s_mean']:>8}"
                    f"{med['ttft_ms_mean']:>8}{med['tok_per_s_mean']:>8}")
        else:
            nums = f"{sp['ttft_ms_mean']:>9}{sp['tok_per_s_mean']:>8}"
        print(f"{r['name']:<20}{nums}{q}")
    print(f"\nresults: {out or (HERE / 'results.json')}")


if __name__ == "__main__":
    main()
