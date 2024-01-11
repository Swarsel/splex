import os
from instance import Instance


def read_input(filename):
    with open(filename, "r") as file:
        edges = []
        name = str(filename)
        line = file.readline()
        (s, n, _, _) = list(map(int, line.strip().split()))
        lines = file.readlines()
        for line in lines:
            edges.append(list(map(int, line.strip().split())))
        #print(edges)
        return Instance(name, s, n, edges)
