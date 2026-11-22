"""Smoke tests for the deterministic checkers. Run: python test_tasks.py"""
import tasks as T


def score(task, r):
    det = [(n, fn(r)) for n, fn in task["checks"]]
    return sum(1 for _, ok in det if ok) / len(det) * 100, det


def task(id_):
    return next(t for t in T.TASKS if t["id"] == id_)


GOOD_SCORES = {}
GOOD = {
    "json_extract": '{"vendor": "Bluepeak Logistics", "due_date": "2027-03-31", "amount": 4132.5}',
    "constraints": "Hybrid systems cut diesel use by 61 percent. Inverters failed about once every 18 months. Field work covered 14 sites. Local owners saw higher uptime than outside operators.",
    "math_word": "Draft 2000/200=10. Quality 2000/100=20. Reprint 200/100=2. ANSWER: 32",
    "reasoning": "Only Ana, Ben, Dev, Cara, Elia fits. The middle seat is Dev. ANSWER: Dev",
    "fix_it": (
        "```python\n"
        "def first_duplicates(items):\n"
        "    counts = {}\n"
        "    order = []\n"
        "    for it in items:\n"
        "        counts[it] = counts.get(it, 0) + 1\n"
        "        if counts[it] == 2:\n"
        "            order.append(it)\n"
        "    return order\n"
        "```\n"
    ),
    "coding": (
        "```python\n"
        "def fold_ledger(lines, rates):\n"
        "    txns = []\n"
        "    for raw in lines:\n"
        "        line = raw.strip()\n"
        "        if not line or line.startswith('#'):\n"
        "            continue\n"
        "        if line.startswith('VOID'):\n"
        "            parts = line.split()\n"
        "            if len(parts) != 4:\n"
        "                continue\n"
        "            _, vendor, amount_s, currency = parts\n"
        "            amount = float(amount_s)\n"
        "            for txn in txns:\n"
        "                if txn[3] and txn[0] == vendor and txn[2] == currency and abs(txn[1] - amount) < 1e-9:\n"
        "                    txn[3] = False\n"
        "                    break\n"
        "            continue\n"
        "        _, vendor, amount_s, currency = [p.strip() for p in line.split(',')]\n"
        "        txns.append([vendor, float(amount_s), currency, True])\n"
        "    nets = {}\n"
        "    for vendor, amount, currency, alive in txns:\n"
        "        if alive:\n"
        "            nets[vendor] = nets.get(vendor, 0.0) + amount * rates[currency]\n"
        "    out = [(v, round(n, 2)) for v, n in nets.items() if abs(n) > 1e-9]\n"
        "    out.sort()\n"
        "    return out\n"
        "```"
    ),
}
BAD = {
    "json_extract": "The vendor is Bluepeak, due March 15 2027, 4312.50 USD.",
    "constraints": "Solar microgrids cut diesel use by 61%, and inverters failed every 18 months across 14 sites.",
    "math_word": "The quote says 40 minutes. ANSWER: 40",
    "reasoning": "I think it is Cara. ANSWER: Cara",
    "coding": "def fold_ledger(lines, rates):\n    return []",
    "fix_it": (
        "```python\n"
        "def first_duplicates(items):\n"
        "    seen = []\n"
        "    result = []\n"
        "    for it in items:\n"
        "        if it in seen:\n"
        "            result.append(it)\n"
        "        else:\n"
        "            seen.append(it)\n"
        "    return result\n"
        "```\n"
    ),
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
