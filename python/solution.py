from graph import Graph
from instance import Instance
from random import shuffle
import numpy as np

class Solution:

    def __init__(self, instance: Instance):
        self.instance = instance
        self.graph = Graph(instance)
        self.node_splex = self.set_feasible_nodes()
        self.cost = self.compute_cost()

    def compute_cost(self):
        cost = 0
        for i in range(1, self.instance.n + 1):
            for j in range(1, self.instance.n + 1):
                if self.edge_differs_from_initial(i, j):
                    cost += abs(self.instance.get_edge_weight(i, j))
        return cost

    def edge_differs_from_initial(self, i, j):
        return self.instance.get_edge_status(i, j) != self.graph.get_edge_status(i, j)

    def is_feasible_solution(self):
        for node in range(1, self.instance.n + 1):
            if not self.node_splex(node):
                return False
        return True

    def is_feasible_node(self, i):
        return self.graph.get_node_degree(i) >= len(self.graph.get_node_component(i)) - self.instance.s

    def get_component_required_degree(self, component):
        return len(component) - self.instance.s

    def is_feasible_component(self, component):
        for node in component:
            if self.node_splex[node - 1] != 1:
                return False
        return True

    def set_feasible_nodes(self):
        splex_tracker = np.zeros(self.instance.n)
        for node in range(1, self.instance.n + 1):
            if self.is_feasible_node(node):
                splex_tracker[node - 1] = 1
        return splex_tracker

    def construct(self):
        components = list(self.graph.get_components())
        shuffle(components)
        for component in components:
            if not self.is_feasible_component(component):
                self.graph.repair_component(component)

    def repair_component(self, component, threshold=0.7):
        required_degree = self.get_component_required_degree(component)
        avg_degree = self.graph.get_component_avg_degree_from_component(component)
        if avg_degree > threshold * required_degree:
            self.graph.add_edges(component)

    def add_edge(self, i, j):
        if not self.graph.get_edge_status(i, j):
            if self.edge_differs_from_initial(i, j):
                self.cost -= abs(self.instance.weights[i - 1][j - 1])
            else:
                self.cost += abs(self.instance.weights[i - 1][j - 1])
            self.graph.add_edge(i, j)

    def remove_edge(self, i, j):
        if self.graph.get_edge_status(i, j):
            if self.edge_differs_from_initial(i, j):
                self.cost -= abs(self.instance.weights[i - 1][j - 1])
            else:
                self.cost += abs(self.instance.weights[i - 1][j - 1])
            self.graph.remove_edge(i, j)

    def add_edges(self, component):
        while not self.is_feasible_component(component):
            for node in component:
                pass


    def __str__(self):
        return f"Solution cost: {self.cost}"
