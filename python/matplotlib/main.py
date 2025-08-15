from itertools import repeat
from string import ascii_lowercase, digits

import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits.mplot3d import Axes3D

fig = plt.figure()
ax1 = fig.add_subplot(111, projection=Axes3D.name)

ROWS = 32
COLS = len(ascii_lowercase + digits)
xpos = [x for y in range(COLS) for x in range(ROWS)]
ypos = [y for x in range(ROWS) for y in repeat(x, COLS)]
zpos = np.zeros(ROWS * COLS)

dx = np.ones(len(xpos))
dy = np.ones(len(ypos))
dz = np.zeros(ROWS * COLS)
dz[17] = 2
dz[19] = 3

ax1.bar3d(xpos, ypos, zpos, dx, dy, dz, color='#00ceaa')
plt.show()
