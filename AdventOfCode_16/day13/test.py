inp = 1350 # YIMV

def clear(x, y):
    h = x*x + 3*x + 2*x*y + y + y*y + inp
    j = bin(h)[2:].count('1')
    return (j % 2) == 0

vis = set()

def adj(cur):
    ret = []
    for spot in cur:
        x, y = (i for i in spot)
        if x != 0:
            if (x - 1, y) not in vis and clear(x - 1, y):
                vis.add(tuple([x - 1, y]))
                ret.append(tuple([x - 1, y]))
        if y != 0:
            if (x, y - 1) not in vis and clear(x, y - 1):
                vis.add(tuple([x, y - 1]))
                ret.append(tuple([x, y - 1]))
        if (x + 1, y) not in vis and clear(x + 1, y):
            vis.add(tuple([x + 1, y]))
            ret.append(tuple([x + 1, y]))
        if (x, y + 1) not in vis and clear(x, y + 1):
            vis.add(tuple([x, y + 1]))
            ret.append(tuple([x, y + 1]))
    return ret


steps = 0
p2 = 0
here = [(1, 1)]
while (31, 39) not in here:
    steps += 1
    here = adj(here)
    if steps == 50:
        p2 = len(vis)
print("Part 2:", p2)