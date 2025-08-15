#! /usr/bin/env python3

"""
>>> import numpy as np
>>> wow = np.loadtxt('wow')
>>> router = np.loadtxt('router')
>>> np.corrcoef(wow, router)
"""

import math

router = [float(line) for line in open('router')]
wow = [float(line) for line in open('wow')]
pearson = [(x, y, x * y, x * x, y * y) for x, y in zip(router, wow)]

sums = [sum(x) for x in zip(*pearson)]

x, y, xy, xx, yy = sums

N = len(pearson)

r = (N*xy-(x*y)) / math.sqrt(math.fabs(N*xx - xx)*math.fabs(N*yy - yy))

print(r)
