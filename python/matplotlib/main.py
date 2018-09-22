import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits.mplot3d import Axes3D

fig = plt.figure()
ax1 = fig.add_subplot(111, projection=Axes3D.name)

xpos = list(range(10)) * 10
ypos = [int(x / 10) for x in range(100)]
zpos = np.zeros(100)

dx = np.ones(len(xpos))
dy = np.ones(len(ypos))
dz = np.zeros(100)
dz[17] = 2
dz[19] = 3

ax1.bar3d(xpos, ypos, zpos, dx, dy, dz, color='#00ceaa')
plt.show()
