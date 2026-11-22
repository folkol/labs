"""Sample `nvidia-smi dmon` during a trial and flag VRAM paging.

A model that fits stays compute-bound: high SM, modest PCIe. One that does not
fit keeps the bus busy moving weights while SM stays low. The first seconds of
a trial are the load, so those samples are ignored. Idle gaps (scoring, the
next prompt) are ignored too; only seconds where the GPU is actually busy count.
"""
import statistics
import subprocess
import threading
import time
from contextlib import contextmanager

# Observed on this 5090 with a resident model: ~85% SM and ~1.3 GB/s combined
# while decoding, and ~6% SM with ~1 GB/s while idle. Paging sits well above that.
SM_LOW = 40
BUS_HIGH_MBS = 4000
ACTIVE_SM = 15
ACTIVE_BUS_MBS = 1000
SKIP_S = 3.0
MIN_ACTIVE = 3
SPILL_FRACTION = 0.6

DMON = ["nvidia-smi", "dmon", "-i", "0", "-s", "putcm", "-d", "1"]


class GpuWatch:
    def __init__(self):
        self.proc = None
        self.results = {}
        self._samples = []
        self._lock = threading.Lock()
        self._thread = None

    def __enter__(self):
        try:
            self.proc = subprocess.Popen(
                DMON, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True,
            )
        except FileNotFoundError:
            print("  nvidia-smi not found; VRAM check skipped", flush=True)
            return self
        self._thread = threading.Thread(target=self._read, daemon=True)
        self._thread.start()
        return self

    def __exit__(self, *exc):
        proc = self.proc
        if proc is not None and proc.poll() is None:
            proc.terminate()
            try:
                proc.wait(timeout=3)
            except subprocess.TimeoutExpired:
                proc.kill()
        if self._thread is not None:
            self._thread.join(timeout=3)
        return False

    def _read(self):
        header = None
        for line in self.proc.stdout:
            line = line.strip()
            if not line:
                continue
            if line.startswith("#"):
                names = line.lstrip("#").split()
                if "sm" in names and "rxpci" in names:
                    header = {name: i for i, name in enumerate(names)}
                continue
            if header is None:
                continue
            parts = line.split()
            sm = _num(parts, header.get("sm"))
            rx = _num(parts, header.get("rxpci"))
            tx = _num(parts, header.get("txpci"))
            if sm is None:
                continue
            with self._lock:
                self._samples.append((time.perf_counter(), sm, rx or 0.0, tx or 0.0))

    @contextmanager
    def span(self, label):
        """Time one trial. Repeated spans with the same label are one combo."""
        if self.proc is None:
            yield
            return
        t0 = time.perf_counter()
        try:
            yield
        finally:
            verdict = classify(self._window(t0, time.perf_counter()))
            if verdict is not None:
                self._merge(label, verdict)

    def spilled(self, label):
        info = self.results.get(label)
        if info and info["spill"]:
            return info
        return None

    def _window(self, t0, t1):
        with self._lock:
            rows = list(self._samples)
        # A line arrives at the end of its 1s interval, so keep one past the end.
        return [(ts - t0, sm, rx, tx) for ts, sm, rx, tx in rows if t0 < ts <= t1 + 1.0]

    def _merge(self, label, verdict):
        prev = self.results.get(label)
        if prev is None:
            self.results[label] = verdict
            return
        if verdict["spill"] and (not prev["spill"] or verdict["bus_mbs"] > prev["bus_mbs"]):
            self.results[label] = verdict
        elif not prev["spill"] and not verdict["spill"] and verdict["active_samples"] > prev["active_samples"]:
            self.results[label] = verdict


def classify(samples):
    """samples are (seconds_into_trial, sm%, rx MB/s, tx MB/s). None when too brief to judge."""
    steady = [s for s in samples if s[0] >= SKIP_S]
    active = [s for s in steady if s[1] >= ACTIVE_SM or s[2] + s[3] >= ACTIVE_BUS_MBS]
    if len(active) < MIN_ACTIVE:
        return None
    bad = [s for s in active if s[1] < SM_LOW and s[2] + s[3] >= BUS_HIGH_MBS]
    spill = len(bad) / len(active) >= SPILL_FRACTION
    pool = bad if spill else active
    return {
        "spill": spill,
        "sm_pct": round(statistics.median(s[1] for s in pool)),
        "bus_mbs": round(statistics.median(s[2] + s[3] for s in pool)),
        "active_samples": len(active),
    }


def warning_text(info):
    return f"couldn't fit in VRAM (GPU {info['sm_pct']}%, bus {info['bus_mbs'] / 1000:.1f} GB/s)"


def _num(parts, idx):
    if idx is None or idx >= len(parts) or parts[idx] == "-":
        return None
    try:
        return float(parts[idx])
    except ValueError:
        return None
