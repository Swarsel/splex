import numpy as np
import matplotlib.pyplot as plt


class Instance:

    def __init__(self, name, s, n, edges):
        self.name = name
        self.s = s
        self.n = n
        self.edges = []
        self.initial_edges = []
        self.missing_edges = []
        self.connected = np.zeros((n, n), dtype=int)
        self.weights = np.zeros((n, n), dtype=int)
        self.parameters = {"threshold": 0.8, "penalty": 10, "selection_size": 0.7, "offspring_ratio": 0.9}
        self.init_edges(edges)

    def init_edges(self, edges):
        edges.sort()
        for (i, j, e, w) in edges:
            if e:
                self.connected[i - 1, j - 1], self.connected[j - 1, i - 1] = 1, 1
                self.weights[i - 1, j - 1], self.weights[j - 1, i - 1] = -w, -w
                self.initial_edges.append((-w, i, j))
                self.edges.append((-w, i, j))
            else:
                self.weights[i - 1, j - 1], self.weights[j - 1, i - 1] = w, w
                self.missing_edges.append((w, i, j))
                self.edges.append((w, i, j))
            # input data is shifted by 1, fix it here
            self.weights[i - 1, j - 1] = w
        self.edges.sort()

    def get_edge_status(self, i, j):
        return self.connected[i - 1, j - 1]

    def get_edge_weight(self, i, j):
        return self.weights[i - 1, j - 1]

    def get_missing_edges(self):
        return self.missing_edges

    def get_edges(self):
        return self.edges

    def get_initial_edges(self):
        return self.initial_edges

    def __str__(self):
        out = ""
        out += f"Instance: {self.name}\n"
        out += f"s: {self.s}\n"
        out += f"n: {self.n}\n"
        return out
