from itertools import cycle
from sys import argv, exit

from PIL import Image

width = 150
height = 150
NAME = cycle("@officer")

if len(argv) != 2:
    print('usage: python art.py image.jpg')
    exit(1)

image = Image.open(argv[1])
image = image.convert('L')  # grayscale
image.thumbnail((height, width), Image.ANTIALIAS)

avg = max(image.getdata()) / 2
data = image.load()
for y in range(image.height):
    for x in range(image.width):
        if data[x, y] < avg:
            print(next(NAME), end='')
        else:
            print(' ', end='')
    print()
