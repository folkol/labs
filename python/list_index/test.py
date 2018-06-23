import time

# words_file = sys.argv[1]
words_file = 'words.txt'
# shuffled_words_file = sys.argv[2]
shuffled_words_file = 'shuffled_words.txt'

with open(words_file) as f:
    words = f.readlines()

with open(shuffled_words_file) as f:
    shuffled_words = f.readlines()

begin = time.time()
for word in shuffled_words:
    words.index(word)
elapsed = time.time() - begin

print(format(elapsed, 'f'))
