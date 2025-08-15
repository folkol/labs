from collections import defaultdict
from random import randrange

N = 52
NUM_ITERATIONS = 1_000_000


def shuffle(xs):
    for i, _ in enumerate(xs):
        j = randrange(len(xs))
        xs[i], xs[j] = xs[j], xs[i]


counter = defaultdict(int)
for _ in range(NUM_ITERATIONS):
    cards = list(range(N))
    shuffle(cards)
    counter[cards[0]] += 1

for permutation, count in sorted(counter.items()):
    print(permutation, count)
