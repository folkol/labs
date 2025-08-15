import time

with open('words.txt') as f:
    words = [line.strip() for line in f]

with open('shuffled_words.txt') as f:
    shuffled_words = [line.strip() for line in f]

begin = time.time()
for word in shuffled_words:
    index = words.index(word)
    #print('Found', word, 'at', index)
elapsed = time.time() - begin

print(f'Took {elapsed} seconds')


