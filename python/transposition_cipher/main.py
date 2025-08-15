plaintext = 'WE ARE DISCOVERED. FLEE AT ONC'.replace(' ', '').replace('.', '')
key = 'ZEBRAS'

chars = plaintext
blocks = []
while chars:
    block, chars = chars[:len(key)], chars[len(key):]
    blocks.append(block)

print('blocks', list(blocks))
for block in blocks:
    print(block)

transposed = []
key_indexof = sorted(key, key=ord)
for c in key_indexof:
    index = key_indexof.index(c)
    word = ''
    for block in blocks:
        word += block[index:index + 1]
    transposed.append(word)
print('transposed', transposed)
for word in transposed:
    print(word)

cipher_blocks = [transposed[key.index(c)] for c in key_indexof]
print('cipher_blocks', cipher_blocks)

cipher = ''.join(cipher_blocks)
print('cipher', cipher)

cipher_block_size = len(plaintext) // len(key)
print(cipher_block_size)

chars = cipher
blocks = []
while chars:
    block, chars = chars[:cipher_block_size], chars[cipher_block_size:]
    blocks.append(block)

print('blocks', blocks)
for block in blocks:
    print(block)
