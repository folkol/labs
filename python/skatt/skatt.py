from collections import defaultdict

import matplotlib.pyplot as plt
from matplotlib.pyplot import figure
import matplotlib.patches as mpatches
figure(num=None, figsize=(8, 6), dpi=500, facecolor='w', edgecolor='k')

categories = defaultdict(lambda: defaultdict(list))
with open('/Users/folkol/Documents/skatt.dat') as f:
    for line in f:
        value, post = line.split('\t')
        category, post = post.strip().split('/', maxsplit=1)
        categories[category]['x'].append(post)
        categories[category]['y'].append(int(value))

print(sum(categories['Staten']['y']))
print(sum(categories['Kommunen']['y']))
print(sum(categories['Ålderspensionen']['y']))
print(sum(categories['Landstinget']['y']))
print(sum(categories['Övrigt']['y']))
# plt.grid(zorder=0)
xs = categories['Staten']['x'] + categories['Kommunen']['x'] + categories['Ålderspensionen']['x'] + \
     categories['Landstinget']['x'] + categories['Övrigt']['x']
ys = categories['Staten']['y'] + categories['Kommunen']['y'] + categories['Ålderspensionen']['y'] + \
     categories['Landstinget']['y'] + categories['Övrigt']['y']
barlist = plt.bar(xs, ys)
plt.xticks(fontsize=4, rotation='vertical')
plt.title('Skatt (474200)')
i = 0
for _ in range(len(categories['Staten']['y'])):
    barlist[i].set_color('r')
    i += 1
for _ in range(len(categories['Kommunen']['y'])):
    barlist[i].set_color('g')
    i += 1
for _ in range(len(categories['Ålderspensionen']['y'])):
    barlist[i].set_color('b')
    i += 1
for _ in range(len(categories['Landstinget']['y'])):
    barlist[i].set_color('y')
    i += 1
for _ in range(len(categories['Övrigt']['y'])):
    barlist[i].set_color('c')
    i += 1
plt.legend(handles=[mpatches.Patch(color='red', label='Staten (159400)'), mpatches.Patch(color='green', label='Kommunen (139200)'), mpatches.Patch(color='blue', label='Ålderspensionen (89500)'), mpatches.Patch(color='yellow', label='Landstringet (78000)'), mpatches.Patch(color='cyan', label='Övrigt (8300)')])
plt.show()
