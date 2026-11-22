"""Smoke tests for the deterministic checkers. Run: python test_tasks.py"""
import tasks as T


def score(task, r):
    det = [(n, fn(r)) for n, fn in task["checks"]]
    return sum(1 for _, ok in det if ok) / len(det) * 100, det


def task(id_):
    return next(t for t in T.TASKS if t["id"] == id_)


GOOD_SCORES = {}
GOOD = {
    "json_extract": '{"vendor": "Bluepeak Logistics", "due_date": "2027-03-15", "amount": 4312.5}',
    "constraints": "Solar microgrids cut diesel use by 61%. Inverter faults averaged one per 18 months, mostly from heat. Local ownership correlated with higher uptime. The study covered 14 sites across three countries. Governance proved more important than hardware choice.",
    "math_word": "Draft: 200*0.6=120. Full: 100*0.4=40. Rate=160. 4800/160=30. ANSWER: 30",
    "reasoning": "Ben must be seat 2, Cara is interior, so the middle is her. ANSWER: Cara",
    "coding": (
        "```python\n"
        "def split_words(text, max_len):\n"
        "    words = text.split()\n"
        "    return [w[i:i+max_len] for w in words for i in range(0, len(w), max_len)]\n"
        "```"
    ),
}
BAD = {
    "json_extract": "The vendor is Bluepeak, due March 15 2027, 4312.50 USD.",
    "constraints": "Solar microgrids! We utilize solar. A",
    "math_word": "It is about 42 minutes. ANSWER: 42",
    "reasoning": "I think it is Dev. ANSWER: Dev",
    "coding": "def split_words(text, max_len):\n    return text.split()",
}

fails = 0
for tid, resp in GOOD.items():
    s, det = score(task(tid), resp)
    ok = s == 100.0
    GOOD_SCORES[tid] = s
    if not ok:
        fails += 1
    print(("ok" if ok else "FAIL"), "GOOD", tid, f": {s:.0f}", det)

for tid, resp in BAD.items():
    s, det = score(task(tid), resp)
    ok = s < GOOD_SCORES[tid]
    if not ok:
        fails += 1
    print(("ok" if ok else "FAIL"), "BAD", tid, f": {s:.0f}", det)

raise SystemExit(fails)
