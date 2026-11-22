"""Task definitions. Each task is either:
  - deterministic: `checks` = list of (name, fn(response) -> bool); score = pass_rate * 100
  - judged: `criteria` = list of (name, description); judge scores each 0/1/2;
    score = sum / (2 * len(criteria)) * 100
"""
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path


def _first_json_block(text):
    m = re.search(r"\{.*\}", text, re.DOTALL)
    if not m:
        return None
    try:
        return json.loads(m.group(0))
    except json.JSONDecodeError:
        return None


def _sentences(r):
    return [s.strip() for s in re.split(r"(?<=[.!?])\s+", r.strip()) if s.strip()]


def _answer_line(r):
    matches = re.findall(r"answer\s*:\s*([^\n]*)", r, re.I)
    return matches[-1].strip() if matches else None


# ---------- json_extract ----------

def _check_json_parses(r):
    return _first_json_block(r) is not None


def _check_vendor(r):
    j = _first_json_block(r)
    return bool(j) and "bluepeak" in str(j.get("vendor", "")).lower()


def _check_due_date(r):
    j = _first_json_block(r)
    return bool(j) and "2027-03-15" in str(j.get("due_date", ""))


def _check_amount(r):
    j = _first_json_block(r)
    return bool(j) and str(j.get("amount", "")).replace(",", "") in ("4312.5", "4312.50", "4312")


# ---------- constraints ----------

def _check_sentence_count(r):
    return 4 <= len(_sentences(r)) <= 6


def _check_no_utilize(r):
    return "utilize" not in r.lower()


def _check_no_exclaim(r):
    return "!" not in r


def _check_starts_solar(r):
    s = _sentences(r)
    return bool(s) and s[0].lower().startswith("solar")


def _check_all_periods(r):
    s = _sentences(r)
    return bool(s) and all(x.rstrip().endswith(".") for x in s)


# ---------- math ----------

def _check_math_answer_present(r):
    return _answer_line(r) is not None


def _check_math_correct(r):
    a = _answer_line(r)
    if not a:
        return False
    nums = re.findall(r"-?\d[\d,]*(?:\.\d+)?", a.replace(",", ""))
    return bool(nums) and abs(float(nums[-1]) - 30.0) < 0.01


# ---------- coding ----------

_CODE_TESTS = r'''
import importlib.util
spec = importlib.util.spec_from_file_location("sol", {path!r})
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
f = mod.split_words
assert f("hello world", 3) == ["hel", "lo", "wor", "ld"], f("hello world", 3)
assert f("ab cd", 2) == ["ab", "cd"], f("ab cd", 2)
assert f("", 3) == [], f("", 3)
assert f("  a   bbbb  ", 2) == ["a", "bb", "bb"], f("  a   bbbb  ", 2)
assert f("python", 7) == ["python"], f("python", 7)
print("PASS")
'''


def _check_code_present(r):
    return "def split_words" in r


def _check_tests_pass(r):
    m = re.search(r"```(?:python)?\s*(.*?)(?:```|\Z)", r, re.DOTALL)
    src = (m.group(1) if m else r).strip()
    if "def split_words" not in src:
        return False
    with tempfile.TemporaryDirectory() as d:
        path = Path(d) / "sol.py"
        path.write_text(src, encoding="utf-8")
        p = subprocess.run(
            [sys.executable, "-I", "-c", _CODE_TESTS.format(path=str(path))],
            capture_output=True, text=True, timeout=30,
        )
        return p.returncode == 0 and "PASS" in p.stdout


# ---------- reasoning ----------
# Ana at left end; Ana next to Ben -> Ben seat 2. Cara interior -> seat 3 or 4.
# Dev left of Elia fills the rest. Middle seat is Cara either way (unique).

def _check_reasoning_present(r):
    return _answer_line(r) is not None


def _check_reasoning_correct(r):
    a = _answer_line(r)
    return bool(a) and a.lower().rstrip(".!?") == "cara"


# ---------- task list ----------

TASKS = [
    {
        "id": "json_extract",
        "prompt": """Extract the fields from this invoice email as JSON with exactly these keys: "vendor", "due_date" (ISO format), "amount" (number, USD).
Email:
---
Hi, attached is our invoice #4471 from Bluepeak Logistics. Total due: $4,312.50. Payment is due by March 15, 2027. Please reference the PO on file. Thanks, Dana
---
Respond with only the JSON object, no prose.""",
        "checks": [
            ("json_parses", _check_json_parses),
            ("vendor", _check_vendor),
            ("due_date", _check_due_date),
            ("amount", _check_amount),
        ],
    },
    {
        "id": "constraints",
        "prompt": """Summarize the passage in exactly 4-6 sentences. Rules:
- every sentence must end with a period
- do not use the word "utilize"
- do not use any exclamation marks
- sentence 1 must begin with the word "Solar"
Passage:
Solar microgrids are increasingly deployed in rural regions where grid extension is uneconomic. A 2026 field study across 14 sites in three countries found that hybrid solar-plus-storage systems reduced diesel consumption by 61% on average. Failure rates were highest in battery inverters, averaging one fault per 18 months, and most failures were attributed to heat, not cold, contrary to earlier assumptions. Communities that owned their microgrids locally reported higher uptime than operator-managed ones, suggesting governance matters more than hardware choice.""",
        "checks": [
            ("sentence_count", _check_sentence_count),
            ("no_utilize", _check_no_utilize),
            ("no_exclaim", _check_no_exclaim),
            ("starts_solar", _check_starts_solar),
            ("all_periods", _check_all_periods),
        ],
    },
    {
        "id": "math_word",
        "prompt": """A printer can print 200 pages per minute at draft mode or 100 pages per minute at full quality. A batch of 4800 pages is printed with 60% of the time spent in draft mode and 40% in full quality. How many minutes does the batch take?
Show the arithmetic, then give the final answer on a line starting with "ANSWER:".""",
        "checks": [
            ("answer_present", _check_math_answer_present),
            ("answer_correct", _check_math_correct),
        ],
    },
    {
        "id": "coding",
        "prompt": """Write a Python function `split_words(text, max_len)` that splits a string into words (whitespace-delimited) and returns a list where every word longer than max_len is broken into chunks of exactly max_len characters (last chunk shorter is fine). Words at or under max_len pass through unchanged. Multiple consecutive spaces should be collapsed to single spaces before splitting. An empty string returns an empty list.
Respond with a single Python code block containing just the function.""",
        "checks": [
            ("code_present", _check_code_present),
            ("tests_pass", _check_tests_pass),
        ],
    },
    {
        "id": "reasoning",
        "prompt": """Five friends — Ana, Ben, Cara, Dev, Elia — sit in a row in some order. Ana sits at the left end. Ana sits next to Ben. Cara does not sit at either end. Dev sits to the left of Elia. Who sits in the middle seat?
Give your reasoning, then the single name on a line starting with "ANSWER:".""",
        "checks": [
            ("answer_present", _check_reasoning_present),
            ("answer_correct", _check_reasoning_correct),
        ],
    },
    {
        "id": "summarize",
        "prompt": """Summarize the following in 2-3 sentences.
A city replaced 12% of its streetlights with smart-dimming LEDs and paired the change with a curfew on non-emergency night work. Electricity use for lighting fell 34%, but reported nighttime traffic incidents rose 9% in the first two quarters. A follow-up audit found the incident increase concentrated on three arterials where dimming had been set 40% lower than the city standard; after those three corridors were brightened, the incident rate returned to baseline within one quarter. The city now treats dimming level as a safety parameter, not a cost lever.
Write the summary only.""",
        "criteria": [
            ("accuracy", "States only facts supported by the passage (no invented numbers or causes)"),
            ("coverage", "Captures the dimming/LED change, the incident spike, and the resolution (brightening the 3 corridors)"),
            ("concision", "2-3 sentences, no filler"),
        ],
    },
    {
        "id": "rewrite_tone",
        "prompt": """Rewrite this email as a calm, professional message to a vendor. Keep every factual detail (dates, amounts, item references) but remove all blame and caps.
'DEADLINE WAS YESTERDAY AND YOUR DELIVERY IS STILL MISSING. ITEM #A-118, $2,140. WE NEED IT BY THURSDAY OR WE ARE CANCELLING THE ENTIRE CONTRACT. THIS IS YOUR SECOND LATE DELIVERY THIS QUARTER.'""",
        "criteria": [
            ("facts_preserved", "Keeps item #A-118, $2,140, Thursday deadline, and the cancellation consequence"),
            ("tone", "Professional and non-accusatory; no caps, no blame language"),
            ("action_clarity", "The required action and deadline are unambiguous"),
        ],
    },
]
