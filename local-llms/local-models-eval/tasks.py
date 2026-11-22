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
    if not j:
        return False
    v = str(j.get("vendor", "")).lower()
    return "bluepeak" in v and "northline" not in v


def _check_due_date(r):
    j = _first_json_block(r)
    return bool(j) and "2027-03-31" in str(j.get("due_date", ""))


def _check_amount(r):
    j = _first_json_block(r)
    if not j:
        return False
    try:
        return abs(float(str(j.get("amount", "")).replace(",", "")) - 4132.5) < 0.001
    except ValueError:
        return False


# ---------- constraints ----------

def _check_sentence_count(r):
    return len(_sentences(r)) == 4


def _word_count(sentence):
    return len(sentence.strip().rstrip(".!?").split())


def _check_word_limit(r):
    s = _sentences(r)
    return bool(s) and all(_word_count(x) <= 16 for x in s)


def _check_numbers_split(r):
    found = {}
    for i, sentence in enumerate(_sentences(r)):
        for n in ("61", "18", "14"):
            if n not in found and re.search(rf"\b{n}\b", sentence):
                found[n] = i
    return len(set(found.values())) == 3


def _check_no_and(r):
    return re.search(r"\band\b", r, re.I) is None


def _check_no_solar(r):
    return "solar" not in r.lower()


def _check_no_comma(r):
    return "," not in r


def _check_diesel_first(r):
    s = _sentences(r)
    return bool(s) and "diesel" in s[0].lower() and re.search(r"\b61\b", s[0]) is not None


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
    return bool(nums) and abs(float(nums[-1]) - 32.0) < 0.01


# ---------- coding ----------

_CODE_TESTS = r'''
import importlib.util
spec = importlib.util.spec_from_file_location("sol", __PATH__)
mod = importlib.util.module_from_spec(spec); spec.loader.exec_module(mod)
f = mod.fold_ledger
rates = {"USD": 1.0, "EUR": 2.0}
assert f([
    "2027-03-01, Bluepeak, 10, USD",
    "2027-03-02, Bluepeak, 10, USD",
    "VOID Bluepeak 10 USD",
], rates) == [("Bluepeak", 10.0)], f([
    "2027-03-01, Bluepeak, 10, USD",
    "2027-03-02, Bluepeak, 10, USD",
    "VOID Bluepeak 10 USD",
], rates)
assert f([
    "2027-03-01, Bluepeak, 10, USD",
    "VOID Other 10 USD",
], rates) == [("Bluepeak", 10.0)]
assert f(["", "  # ignore", "   "], rates) == []
assert f([
    "2027-03-01, Bluepeak, 10, USD",
    "2027-03-02, Bluepeak, 10, EUR",
    "2027-03-03, Acme, 5, USD",
], rates) == [("Acme", 5.0), ("Bluepeak", 30.0)]
assert f([
    "2027-03-01, Bluepeak, 10, USD",
    "VOID Bluepeak 10 USD",
    "2027-03-04, Acme, 5, USD",
], rates) == [("Acme", 5.0)]
print("PASS")
'''


def _check_code_present(r):
    return "def fold_ledger" in r


def _check_tests_pass(r):
    m = re.search(r"```(?:python)?\s*(.*?)(?:```|\Z)", r, re.DOTALL)
    src = (m.group(1) if m else r).strip()
    if "def fold_ledger" not in src:
        return False
    with tempfile.TemporaryDirectory() as d:
        path = Path(d) / "sol.py"
        path.write_text(src, encoding="utf-8")
        p = subprocess.run(
            [sys.executable, "-I", "-c", _CODE_TESTS.replace("__PATH__", repr(str(path)))],
            capture_output=True, text=True, timeout=30,
        )
        return p.returncode == 0 and "PASS" in p.stdout


# ---------- reasoning ----------
# 1 Ana, 2 Ben. Cara is 3 or 4. Elia adjacent to Cara. Dev left of Elia and
# not at an end. Only Ana, Ben, Dev, Cara, Elia works, so the middle is Dev.

def _check_reasoning_present(r):
    return _answer_line(r) is not None


def _check_reasoning_correct(r):
    a = _answer_line(r)
    return bool(a) and a.lower().rstrip(".!?") == "dev"


# ---------- task list ----------

TASKS = [
    {
        "id": "json_extract",
        "prompt": """Extract one JSON object with exactly these keys: "vendor", "due_date" (ISO format), "amount" (number, USD, the amount still owed on this invoice).
Email thread:
---
From: Dana, Bluepeak Logistics
Attached is invoice #4471. Total: $4,312.50. I put the due date as March 15, 2027. The shipment was handed to Northline Freight.

From: Accounts payable
The goods were received on 1 March 2027. The contract says payment is due 30 days after receipt, and that due date replaces the one printed on the invoice. Apply credit memo CM-19 for $180.00 against invoice 4471. Northline's freight bill is a separate invoice and is not part of this balance.
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
        "prompt": """Summarize the passage in exactly 4 sentences. Rules:
- every sentence ends with a period
- every sentence is at most 16 words
- the digits 61, 18, and 14 each appear in a different sentence
- sentence 1 states the diesel reduction
- do not use the word "and"
- do not use the word "solar"
- do not use commas
Passage:
Solar microgrids are increasingly deployed in rural regions where grid extension is uneconomic. A 2026 field study across 14 sites in three countries found that hybrid solar-plus-storage systems reduced diesel consumption by 61% on average. Failure rates were highest in battery inverters, averaging one fault per 18 months, and most failures were attributed to heat, not cold, contrary to earlier assumptions. Communities that owned their microgrids locally reported higher uptime than operator-managed ones, suggesting governance matters more than hardware choice.""",
        "checks": [
            ("sentence_count", _check_sentence_count),
            ("word_limit", _check_word_limit),
            ("numbers_split", _check_numbers_split),
            ("no_and", _check_no_and),
            ("no_solar", _check_no_solar),
            ("no_comma", _check_no_comma),
            ("diesel_first", _check_diesel_first),
            ("all_periods", _check_all_periods),
        ],
    },
    {
        "id": "math_word",
        "prompt": """A shop must produce 4000 finished pages. Half of those pages are printed in draft mode at 200 pages per minute, and half in full quality at 100 pages per minute. After that first pass, 10% of the full-quality pages fail inspection and are reprinted once, again at full quality. The customer's quote assumed the job would take 40 minutes; that quote is not an input.
How many minutes does the work take?
Show the arithmetic, then give the final answer on a line starting with "ANSWER:".""",
        "checks": [
            ("answer_present", _check_math_answer_present),
            ("answer_correct", _check_math_correct),
        ],
    },
    {
        "id": "coding",
        "prompt": """Write a Python function `fold_ledger(lines, rates)`.

`lines` is a list of strings. `rates` maps a currency code to how many USD one unit of that currency is worth.

Skip a line if it is blank or if, after stripping, it starts with `#`.
A transaction line is `DATE, VENDOR, AMOUNT, CURRENCY`, split on commas into exactly four fields, then stripped. `AMOUNT` is a number. `VENDOR` does not contain a comma.
A void line is `VOID VENDOR AMOUNT CURRENCY`, split on whitespace. It removes the earliest still-active transaction with that vendor, amount, and currency. A void that matches nothing is ignored.
The net for a vendor is the sum of its remaining amounts converted to USD (`amount * rates[currency]`), rounded to 2 decimal places. Omit vendors whose net is 0.
Return a list of `(vendor, net_usd)` pairs sorted by vendor name.

Respond with a single Python code block containing just the function.""",
        "checks": [
            ("code_present", _check_code_present),
            ("tests_pass", _check_tests_pass),
        ],
    },
    {
        "id": "reasoning",
        "prompt": """Five friends — Ana, Ben, Cara, Dev, Elia — sit in a row of five seats. Ana sits at the left end. Ana sits next to Ben. Cara does not sit at either end. Elia sits next to Cara. Dev sits somewhere to the left of Elia. Dev does not sit at either end. Who sits in the middle seat?
Give your reasoning, then the single name on a line starting with "ANSWER:".""",
        "checks": [
            ("answer_present", _check_reasoning_present),
            ("answer_correct", _check_reasoning_correct),
        ],
    },
    {
        "id": "summarize",
        "prompt": """Summarize the following in exactly 2 sentences.
A city replaced 12% of its streetlights with smart-dimming LEDs. Electricity use for lighting fell 34%, but reported nighttime traffic incidents rose 9% in the first two quarters. A follow-up audit found the incident increase concentrated on three arterials where dimming had been set 40% lower than the city standard; after those three corridors were brightened, the incident rate returned to baseline within one quarter. The city now treats dimming level as a safety parameter, not a cost lever.
A separate proposal, a curfew on non-emergency night work, was discussed and was not adopted. Citywide nighttime incidents were already 2% above the prior year before any lights were dimmed.
Write the summary only.""",
        "criteria": [
            ("accuracy", "Uses only facts from the passage. Does not present the curfew as adopted. Does not treat the pre-existing 2% rise as an effect of the dimming"),
            ("coverage", "Includes the 34% electricity drop, the 9% incident rise, and the restoration of the three under-dimmed corridors"),
            ("selection", "Omits the rejected curfew, and does not cite the 2% figure as the outcome of the lighting change"),
        ],
    },
    {
        "id": "rewrite_tone",
        "prompt": """Rewrite this as a calm, professional message to the vendor. Keep every contractual fact, including facts that appear only inside an insult. Drop sarcasm, name-calling, and any figure that is not part of the contract. State the cancellation as a consequence, not a threat.

'I cannot believe your useless crew still has not delivered ITEM #A-118. That is $2,140 sitting on a truck somewhere while we wait. Have it here by Thursday or we cancel the entire contract. This is the second late delivery this quarter, which is exactly what I expect from you people. For $50 we might as well have paid a bike courier.'""",
        "criteria": [
            ("facts_preserved", "Keeps item #A-118, $2,140, the Thursday deadline, the second late delivery this quarter, and the cancellation consequence"),
            ("no_decoy", "Does not mention $50 or a bike courier"),
            ("tone", "Professional, no insults, and the cancellation is a consequence rather than a threat"),
        ],
    },
]
