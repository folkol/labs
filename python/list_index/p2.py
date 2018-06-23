import random
import time
from string import ascii_letters as alphabet

for i in range(1, 11):
    words = [''.join(random.sample(list(alphabet), len(alphabet))) for _ in range(i * 100000)]
    word = words[0]
    random.shuffle(words)

    begin = time.time()
    words.index(word)
    elapsed = time.time() - begin

    print("{:f}\t".format(elapsed))
