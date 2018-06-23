import time

WORD = u"lorem ipsum dolor sit amet " * 100

for i in range(1, 10):
    n = i * 500000

    words = [u"{} - {}".format(WORD, i) for i in range(n)]

    begin = time.time()
    words.index(words[-1])
    elapsed = time.time() - begin

    print("{:f}\t".format(elapsed))
