import random
from string import ascii_letters as alphabet


def random_word():
    return ''.join(random.sample(alphabet, len(alphabet)))


for i in range(1, 11):
    words = [random_word() for _ in range(i * 100000)]
    with open(f'words_{i}.txt', 'w') as f:
        for word in words:
            f.write(word + '\n')
