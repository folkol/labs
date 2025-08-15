import time

with open('words.txt', 'br') as f:
    words = [line.strip() for line in f]

with open('shuffled_words.txt', 'br') as f:
    shuffled_words = [line.strip() for line in f]

begin = time.time()
for word in shuffled_words:
    index = words.index(word)
    #print(f'Found {word} at {index}')
elapsed = time.time() - begin

print(f'Took {elapsed} seconds')


