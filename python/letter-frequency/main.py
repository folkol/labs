import fileinput
from collections import Counter
import matplotlib.pyplot as plt

words = fileinput.input()
letters = Counter(initial for initial, *_ in words)
letters = dict(sorted(letters.items()))
plt.bar(letters.keys(), letters.values())
plt.show()
