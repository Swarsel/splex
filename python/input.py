import os
from instance import Instance


def read_input(filename):
    with open(filename, "r") as file:
        edges = []
        name = str(filename)
        line = file.readline()
        (s, n, _, _) = list(map(int, line.strip().split()))
        while line := file.readline():
            edges.append(list(map(int, line.strip().split())))
        return Instance(name, s, n, edges)
