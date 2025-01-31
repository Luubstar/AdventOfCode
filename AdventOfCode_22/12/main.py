import numpy as np
from copy import deepcopy
from queue import Queue

DAY = 12


def OK(Map: list[list[int]], i: int, j: int) -> bool:
    return not(i < 0 or j < 0 or i >= len(Map) or j >= len(Map[i]))


def lee(Map: list[list[int]], start: tuple[int, int], end: tuple[int, int]) -> int:
    Dist = [[0 for _ in range(len(Map[j]))] for j in range(len(Map))]
    Dist[start[0]][start[1]] = 1

    Map[start[0]][start[1]] = 'a'
    Map[end[0]][end[1]] = 'z'
    
    Q = Queue()
    Q.put(start)
    
    d = ((0, 1), (0, -1), (1, 0), (-1, 0))

    while not Q.empty():
        i, j = Q.get()

        for k in range(4):
            i_next, j_next = i + d[k][0], j + d[k][1]
            
            if OK(Map, i_next, j_next) and ord(Map[i_next][j_next])-ord(Map[i][j]) <= 1 and Dist[i_next][j_next] < 1:
                Dist[i_next][j_next] = Dist[i][j] + 1

                Q.put((i_next, j_next))

    return Dist[end[0]][end[1]]-1


def reverse_lee(Map: list[list[int]], start: tuple[int, int], end: str) -> int:
    Dist = [[0 for _ in range(len(Map[j]))] for j in range(len(Map))]
    Dist[start[0]][start[1]] = 1

    Map[start[0]][start[1]] = 'z'
    
    Q = Queue()
    Q.put(start)
    
    d = ((0, 1), (0, -1), (1, 0), (-1, 0))
    
    possible = []
    while not Q.empty():
        i, j = Q.get()

        for k in range(4):
            i_next, j_next = i + d[k][0], j + d[k][1]
            
            if OK(Map, i_next, j_next) and ord(Map[i][j])-ord(Map[i_next][j_next]) <= 1 and Dist[i_next][j_next] < 1:
                Dist[i_next][j_next] = Dist[i][j] + 1
                
                if Map[i_next][j_next] == end:
                    possible.append(Dist[i][j])
                
                Q.put((i_next, j_next))
    
    return min(possible)


def part1(inp: list[str]) -> int:
    Map = []
    for line in inp:
        Map.append(list(line))
    
    start = end = ()

    for i in range(len(Map)):
        for j in range(len(Map[i])):
            if Map[i][j] == 'S':
                start = (i, j)
            if Map[i][j] == 'E':
                end = (i, j)
    
    return lee(Map, start, end)


def part2(inp: list[str]) -> int:
    Map = []
    for line in inp:
        Map.append(list(line))
    
    for i in range(len(Map)):
        for j in range(len(Map[i])):
            if Map[i][j] == 'E':
                return reverse_lee(Map, (i, j), 'a')


def read_input_file(filename: str) -> list[str]:
    with open(filename, 'r', encoding='utf8') as fin:
        inp = fin.readlines()

    return [line.strip() for line in inp]


if __name__ == '__main__':
    input_str = read_input_file("input")
    # input_str = read_input_file(f"data/input00.txt")

    print(f"Part 1: {part1(input_str)}")
    print(f"Part 2: {part2(input_str)}")